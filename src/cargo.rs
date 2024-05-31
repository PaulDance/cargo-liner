//! Module handling the execution of cargo for various operations.
//!
//! See [`install_all`] in order to install configured packages.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::{env, iter};

use clap::ColorChoice;
use color_eyre::eyre::{self, eyre, Result, WrapErr};
use color_eyre::Section;
use log::Level;
use regex::Regex;
use semver::Version;

use crate::config::Package;

/// Installs a package, by running `cargo install` passing the `name`, `version`
/// and requested `features`, and returns the exit code of the process.
///
/// The launched process' path is determined using the `$CARGO` environment
/// variable as it is set by Cargo when it calls an external subcommand's
/// corresponding program. See the [Cargo reference] for more details.
///
/// [Cargo reference]: https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands
#[allow(clippy::too_many_arguments)]
fn install(
    name: &str,
    version: &str,
    no_default_features: bool,
    all_features: bool,
    features: &[String],
    index: Option<&str>,
    registry: Option<&str>,
    git: Option<&str>,
    branch: Option<&str>,
    extra_arguments: &[String],
    environment: &BTreeMap<String, String>,
    force: bool,
    color: ColorChoice,
    verbosity: i8,
) -> Result<ExitStatus> {
    let mut cmd = Command::new(env_var()?);

    if !environment.is_empty() {
        cmd.envs(environment);
        log::trace!("Environment set: {:#?}", environment);
    }

    cmd.args(["--color", &color.to_string()]);

    match verbosity.cmp(&0) {
        Ordering::Greater => {
            let opt = iter::once('-')
                .chain(iter::repeat('v').take(verbosity.try_into().unwrap()))
                .collect::<String>();
            cmd.arg(&opt);
            log::trace!("`{opt}` arg added.");
        }
        Ordering::Less => {
            cmd.arg("-q");
            log::trace!("`-q` arg added.");
        }
        Ordering::Equal => {}
    }

    cmd.args(["install", "--version", version]);

    if no_default_features {
        cmd.arg("--no-default-features");
        log::trace!("`--no-default-features` arg added.");
    }

    if all_features {
        cmd.arg("--all-features");
        log::trace!("`--all-features` arg added.");
    }

    if !features.is_empty() {
        cmd.arg("--features").arg(features.join(","));
        log::trace!("`--features` arg added.");
    }

    if let Some(index) = index {
        cmd.args(["--index", index]);
        log::trace!("`--index {}` args added.", index);
    }

    if let Some(registry) = registry {
        cmd.args(["--registry", registry]);
        log::trace!("`--registry {}` args added.", registry);
    }

    if let Some(git) = git {
        cmd.args(["--git", git]);
        log::trace!("`--git {}` args added.", git);
    }

    if let Some(branch) = branch {
        cmd.args(["--branch", branch]);
        log::trace!("`--branch {}` args added.", branch);
    }

    if force {
        cmd.arg("--force");
        log::trace!("`--force` arg added.");
    }

    // This should be kept here: after all other options and before the `--`.
    if !extra_arguments.is_empty() {
        cmd.args(extra_arguments);
        log::trace!("Extra arguments added: {:#?}", extra_arguments);
    }

    cmd.args(["--", name]);
    log_cmd(&cmd);

    cmd.status()
        .wrap_err("Failed to execute Cargo.")
        .note("This can happen for many reasons, but it should not happen easily at this point.")
        .suggestion("Read the underlying error message.")
}

/// Runs `cargo install` for all packages listed in the given user
/// configuration and returns a per-package installation report.
///
/// Returns `Ok(report)` when `keep_going` is `true`, otherwise `Err(err)` of
/// the first error `err` encountered.
pub fn install_all(
    packages: &BTreeMap<String, Package>,
    installed: &BTreeSet<String>,
    keep_going: bool,
    force: bool,
    color: ColorChoice,
    verbosity: i8,
) -> Result<InstallReport> {
    // Returned installation report.
    let mut rep = BTreeMap::new();
    // Aggregation of errors when `keep_going` is enabled.
    let mut err_rep = None::<eyre::Report>;

    for (pkg_name, pkg) in packages {
        let is_installed = installed.contains(pkg_name);
        log::info!(
            "{}ing `{pkg_name}`...",
            if is_installed { "Updat" } else { "Install" }
        );

        if let Err(err) = install(
            pkg_name,
            &pkg.version().to_string(),
            !pkg.default_features(),
            pkg.all_features(),
            pkg.features(),
            pkg.index(),
            pkg.registry(),
            pkg.git(),
            pkg.branch(),
            pkg.extra_arguments(),
            &pkg.environment(),
            force,
            color,
            verbosity,
        )
        .and_then(|status| {
            status.success().then_some(()).ok_or_else(|| {
                let err = eyre!("Cargo process finished unsuccessfully: {status}")
                    .note("This can happen for many reasons.")
                    .suggestion("Read Cargo's output.");

                if keep_going {
                    err
                } else {
                    err.suggestion(
                        "Use `--keep-going` to ignore this and continue on with other packages.",
                    )
                }
            })
        })
        .inspect(|()| {
            rep.insert(
                pkg_name.clone(),
                if is_installed {
                    InstallStatus::Updated
                } else {
                    InstallStatus::Installed
                },
            );
        })
        .wrap_err_with(|| {
            rep.insert(pkg_name.clone(), InstallStatus::Failed);
            format!(
                "Failed to {} {pkg_name:?}.",
                if is_installed { "update" } else { "install" }
            )
        }) {
            if keep_going {
                // Can't use `Option::map_or` for ownership reasons.
                err_rep = Some(match err_rep {
                    Some(err_rep) => err_rep.wrap_err(err),
                    None => err,
                });
            } else {
                return Err(err);
            }
        }
    }

    Ok(InstallReport {
        package_statuses: rep,
        error_report: err_rep,
    })
}

/// Result of [`install_all`].
#[must_use]
#[derive(Debug)]
pub struct InstallReport {
    /// Installation status per package name.
    pub package_statuses: BTreeMap<String, InstallStatus>,
    /// Aggregation of errors to bubble up.
    pub error_report: Option<eyre::Report>,
}

/// Result of [`install`] for some package.
#[derive(Debug)]
pub enum InstallStatus {
    /// The package was newly installed with success.
    Installed,
    /// The package was successfully updated, replacing a previous version.
    Updated,
    /// The package failed to install or update.
    Failed,
}

/// Spawns `cargo search` for the given package with only stdout piped and
/// returns the corresponding child process handle to be used with
/// [`finish_search_exact`].
fn spawn_search_exact(pkg: &str) -> Result<Child> {
    // HACK: detect test environment using some environment variable.
    let is_test = env::var_os("__CARGO_TEST_ROOT").is_some();
    let mut cmd = Command::new(env_var()?);

    cmd.stdin(Stdio::null());
    cmd.stderr(if is_test || log::log_enabled!(Level::Debug) {
        Stdio::inherit()
    } else {
        Stdio::null()
    });
    cmd.stdout(Stdio::piped());
    cmd.args(["--color=never", "search"]);

    if is_test {
        cmd.arg("--registry=dummy-registry");
    }

    cmd.args(["--limit=1", "--", pkg]);

    log_cmd(&cmd);
    cmd.spawn()
        .wrap_err("Failed to spawn Cargo.")
        .note("This can happen for many reasons, but it should not happen easily at this point.")
        .suggestion("Read the underlying error message.")
}

/// Waits for the given child process as spawned by [`spawn_search_exact`] to
/// finish and extract the received package version from the output.
fn finish_search_exact(pkg: &str, proc: Child) -> Result<Version> {
    let out = proc
        .wait_with_output()
        .wrap_err_with(|| format!("Failed to wait for the Cargo child process for {pkg:?}."))
        .note("This can happen for many reasons, but it should not happen easily at this point.")
        .suggestion("Read the underlying error message.")?;

    if !out.status.success() {
        eyre::bail!(
            "Search for {:?} failed on {:?} with stderr: {:?}",
            pkg,
            out.status.code(),
            String::from_utf8(out.stderr),
        );
    }

    let stdout = String::from_utf8(out.stdout)
        .wrap_err("Failed to decode the standard output.")
        .note("This really should not happen.")
        .suggestion(crate::OPEN_ISSUE_MSG)?;
    log::trace!("Search for {pkg:?} got: {stdout:?}");

    // See https://semver.org/#backusnaur-form-grammar-for-valid-semver-versions.
    let ver = Regex::new(&format!(r#"{pkg}\s=\s"([0-9a-zA-Z.+-]+)"\s+#.*"#))
        .wrap_err_with(|| format!("Failed to build the search regex for {pkg:?}."))
        .note("This can happen if the package name contains some special characters.")
        .suggestion("Check that the name is the intended one or open an issue.")?
        .captures(
            stdout
                .lines()
                .next()
                .ok_or_else(|| eyre!("Not at least one line in search output for {pkg:#?}."))
                .suggestion("Check that the package does indeed exist.")?,
        )
        .ok_or_else(|| eyre!("No regex capture while parsing search output for {pkg:#?}."))
        .suggestion("Check that the package does indeed exist.")?
        .get(1)
        .ok_or_else(|| eyre!("Version not captured by regex matching search output for {pkg:#?}."))
        .note(
            "This should not easily happen at this point: the search returned the wanted package.",
        )
        .suggestion(crate::OPEN_ISSUE_MSG)?
        .as_str()
        .parse::<Version>()
        .wrap_err_with(|| format!("Failed to parse the version received for {pkg:?}."))
        .note(
            "This should not easily happen at this point: the search returned the wanted package.",
        )
        .suggestion(crate::OPEN_ISSUE_MSG)?;
    log::trace!("Parsed version is: {ver:#?}.");

    Ok(ver)
}

/// Runs `*_search_exact` for all packages in the given map and returns the
/// thus fetched versions in the collected map.
pub fn search_exact_all(pkgs: &BTreeMap<String, Package>) -> Result<BTreeMap<String, Version>> {
    log::info!("Fetching latest package versions...");
    let mut procs = Vec::new();
    let mut vers = BTreeMap::new();

    log::debug!("Spawning search child processes in parallel...");
    for pkg in pkgs.keys() {
        procs.push(
            spawn_search_exact(pkg)
                .wrap_err_with(|| format!("Failed to spawn search for {pkg:?}."))?,
        );
    }

    log::debug!("Waiting for each search child processes to finish...");
    // Key traversal order is stable because sorted.
    for (pkg, proc) in pkgs.keys().zip(procs.into_iter()) {
        vers.insert(
            pkg.clone(),
            finish_search_exact(pkg, proc)
                .wrap_err_with(|| format!("Failed to finish search for {pkg:?}."))?,
        );
    }

    Ok(vers)
}

/// Runs `cargo config get` with the given configuration key and returns the
/// collected string value.
pub fn config_get(key: &str) -> Result<String> {
    let mut cmd = Command::new(env_var()?);
    // HACK: get access to nightly features.
    // FIXME: remove when `config` gets stabilized.
    cmd.env("RUSTC_BOOTSTRAP", "1");
    cmd.args([
        "--color=never",
        "-Zunstable-options",
        "config",
        "get",
        "--format=json-value",
        "--",
        key,
    ]);

    log_cmd(&cmd);
    let out = cmd
        .output()
        .wrap_err("Failed to execute Cargo.")
        .note("This can happen for many reasons, but it should not happen easily at this point.")
        .suggestion("Read the underlying error message.")?;
    out.status.success().then_some(()).ok_or_else(|| {
        eyre!(
            "Command failed with status: {:#?} and stderr: {:#?}.",
            out.status,
            String::from_utf8(out.stderr),
        )
    })?;

    let out_str = String::from_utf8(out.stdout)
        .wrap_err("Failed to decode the standard output.")
        .note("This really should not happen.")
        .suggestion(crate::OPEN_ISSUE_MSG)?;
    log::trace!("Got: {out_str:#?}.");
    Ok(out_str.trim_end().trim_matches('"').to_owned())
}

/// Wrapper around [`home::cargo_home`] with additional reporting context.
pub fn home() -> Result<PathBuf> {
    home::cargo_home()
        .wrap_err("Failed to get the path to Cargo's home.")
        .suggestion("Check the current directory's permissions and your user OS configuration.")
}

/// Fetches the `CARGO` variable from the environment.
fn env_var() -> Result<String> {
    env::var("CARGO")
        .wrap_err("Failed to get the `CARGO` environment variable.")
        .note("This should not happen easily as it is set by Cargo.")
        .suggestion("Run the command through Cargo as intended.")
}

/// Logs the program and arguments of the given command to DEBUG.
fn log_cmd(cmd: &Command) {
    log::debug!(
        "Running {:#?} with arguments {:#?}...",
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<_>>(),
    );
}

#[cfg(test)]
mod tests {
    use std::sync::Mutex;

    use cargo_test_macro::cargo_test;
    use once_cell::sync::Lazy;

    use super::*;
    use crate::testing;

    const SELF: &str = clap::crate_name!();
    const NONE: &str = "azertyuiop-qsdfghjklm_wxcvbn";
    static LOCK: Lazy<Mutex<()>> = Lazy::new(|| Mutex::new(()));

    #[cargo_test]
    fn test_singlethreaded_searchspawn_self_isok() -> Result<()> {
        let _lk = LOCK.lock();
        let _reg = testing::init_registry();
        testing::fake_publish(SELF, "0.0.0");
        testing::set_env();

        let proc = spawn_search_exact(SELF)?;
        assert_ne!(proc.id(), 0);
        assert!(proc.stdin.is_none());
        assert!(proc.stderr.is_none());
        assert!(proc.stdout.is_some());

        let res = proc.wait_with_output()?;
        assert!(res.status.success());
        assert!(String::from_utf8(res.stdout)?.lines().count() > 0);

        Ok(())
    }

    #[cargo_test]
    fn test_singlethreaded_searchfinish_self_isok() -> Result<()> {
        let _lk = LOCK.lock();
        let _reg = testing::init_registry();
        testing::fake_publish(SELF, "0.0.0");
        testing::set_env();

        assert!(
            finish_search_exact(SELF, spawn_search_exact(SELF)?)?
                <= clap::crate_version!().parse()?
        );
        Ok(())
    }

    #[cargo_test]
    fn test_singlethreaded_searchspawn_none_isok() -> Result<()> {
        let _lk = LOCK.lock();
        let _reg = testing::init_registry();
        testing::set_env();

        let proc = spawn_search_exact(NONE)?;
        assert_ne!(proc.id(), 0);
        assert!(proc.stdin.is_none());
        assert!(proc.stderr.is_none());
        assert!(proc.stdout.is_some());

        let res = proc.wait_with_output()?;
        assert!(res.status.success());
        assert_eq!(String::from_utf8(res.stdout)?.lines().count(), 0);

        Ok(())
    }

    #[cargo_test]
    fn test_singlethreaded_searchfinish_none_iserr() -> Result<()> {
        let _lk = LOCK.lock();
        let _reg = testing::init_registry();
        testing::set_env();

        assert!(finish_search_exact(NONE, spawn_search_exact(NONE)?).is_err());
        Ok(())
    }

    #[cargo_test]
    fn test_singlethreaded_searchall_selfandothers_isok() -> Result<()> {
        let _lk = LOCK.lock();
        let _reg = testing::init_registry();
        testing::fake_publish_all([
            (SELF, clap::crate_version!()),
            ("cargo-expand", "1.0.79"),
            ("cargo-tarpaulin", "0.27.3"),
            ("bat", "0.24.0"),
        ]);
        testing::set_env();

        for (pkg, ver) in search_exact_all(
            &[SELF, "cargo-expand", "cargo-tarpaulin", "bat"]
                .into_iter()
                .map(|pkg| (pkg.to_owned(), Package::SIMPLE_STAR))
                .collect(),
        )? {
            assert_eq!(
                ver,
                match &*pkg {
                    SELF => clap::crate_version!(),
                    "cargo-expand" => "1.0.79",
                    "cargo-tarpaulin" => "0.27.3",
                    "bat" => "0.24.0",
                    pkg => eyre::bail!("Unexpected package: {pkg:?}"),
                }
                .parse()?,
            );
        }
        Ok(())
    }

    #[cargo_test]
    fn test_singlethreaded_searchall_none_iserr() {
        let _lk = LOCK.lock();
        let _reg = testing::init_registry();
        testing::set_env();

        assert!(search_exact_all(
            &[(NONE.to_owned(), Package::SIMPLE_STAR)]
                .into_iter()
                .collect()
        )
        .is_err());
    }

    #[test]
    fn test_configget_withenv_installroot() -> Result<()> {
        env::set_var("CARGO_INSTALL_ROOT", "/tmp");
        assert_eq!(config_get("install.root")?, "/tmp");
        env::remove_var("CARGO_INSTALL_ROOT");
        Ok(())
    }
}

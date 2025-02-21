//! Module handling the execution of cargo for various operations.
//!
//! See [`install_all`] in order to install configured packages.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::OsStr;
use std::path::PathBuf;
use std::process::{Child, Command, ExitStatus, Stdio};
use std::{env, io, iter};

use clap::ColorChoice;
use color_eyre::eyre::{self, eyre, Result, WrapErr};
use color_eyre::Section;
use log::Level;
use regex::Regex;
use semver::Version;

use crate::cli::BinstallChoice;
use crate::config::DetailedPackageReq;

/// Installs a package, by running `cargo install` passing the `name`, `version`
/// and requested `features`, and returns the exit code of the process, if any.
///
/// The launched process' path is determined using the `$CARGO` environment
/// variable as it is set by Cargo when it calls an external subcommand's
/// corresponding program. See the [Cargo reference] for more details.
///
/// [Cargo reference]: https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands
#[expect(
    clippy::too_many_lines,
    reason = "This is just a long list of options to apply."
)]
fn install(
    pkg_name: &str,
    pkg_req: &DetailedPackageReq,
    force: bool,
    dry_run: bool,
    color: ColorChoice,
    verbosity: i8,
) -> Result<Option<ExitStatus>> {
    let mut cmd = Command::new(env_var()?);

    if !pkg_req.environment.is_empty() {
        cmd.envs(&pkg_req.environment);
        log::trace!("Environment set: {:#?}", pkg_req.environment);
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

    cmd.args(["install", "--version", &pkg_req.version.to_string()]);

    if !pkg_req.default_features {
        cmd.arg("--no-default-features");
        log::trace!("`--no-default-features` arg added.");
    }

    if pkg_req.all_features {
        cmd.arg("--all-features");
        log::trace!("`--all-features` arg added.");
    }

    if !pkg_req.features.is_empty() {
        cmd.arg("--features").arg(pkg_req.features.join(","));
        log::trace!("`--features` arg added.");
    }

    if let Some(index) = pkg_req.index.as_deref() {
        cmd.args(["--index", index]);
        log::trace!("`--index {}` args added.", index);
    }

    if let Some(registry) = pkg_req.registry.as_deref() {
        cmd.args(["--registry", registry]);
        log::trace!("`--registry {}` args added.", registry);
    }

    if let Some(git) = pkg_req.git.as_deref() {
        cmd.args(["--git", git]);
        log::trace!("`--git {}` args added.", git);
    }

    if let Some(branch) = pkg_req.branch.as_deref() {
        cmd.args(["--branch", branch]);
        log::trace!("`--branch {}` args added.", branch);
    }

    if let Some(tag) = pkg_req.tag.as_deref() {
        cmd.args(["--tag", tag]);
        log::trace!("`--tag {}` args added.", tag);
    }

    if let Some(rev) = pkg_req.rev.as_deref() {
        cmd.args(["--rev", rev]);
        log::trace!("`--rev {}` args added.", rev);
    }

    if let Some(path) = pkg_req.path.as_deref() {
        cmd.args(["--path", path]);
        log::trace!("`--path {}` args added.", path);
    }

    for bin in &pkg_req.bins {
        cmd.args(["--bin", bin]);
        log::trace!("`--bin {}` args added.", bin);
    }

    if pkg_req.all_bins {
        cmd.arg("--bins");
        log::trace!("`--bins` arg added.");
    }

    for example in &pkg_req.examples {
        cmd.args(["--example", example]);
        log::trace!("`--example {}` args added.", example);
    }

    if pkg_req.all_examples {
        cmd.arg("--examples");
        log::trace!("`--examples` arg added.");
    }

    if pkg_req.ignore_rust_version {
        cmd.arg("--ignore-rust-version");
        log::trace!("`--ignore-rust-version` arg added.");
    }

    if force {
        cmd.arg("--force");
        log::trace!("`--force` arg added.");
    }

    if pkg_req.frozen {
        cmd.arg("--frozen");
        log::trace!("`--frozen` arg added.");
    }

    if pkg_req.locked {
        cmd.arg("--locked");
        log::trace!("`--locked` arg added.");
    }

    if pkg_req.offline {
        cmd.arg("--offline");
        log::trace!("`--offline` arg added.");
    }

    // This should be kept here: after all other options and before the `--`.
    if !pkg_req.extra_arguments.is_empty() {
        cmd.args(&pkg_req.extra_arguments);
        log::trace!("Extra arguments added: {:#?}", pkg_req.extra_arguments);
    }

    cmd.args(["--", pkg_name]);
    log_cmd(&cmd);

    // Keep it as late as possible for fidelity.
    if dry_run {
        // TODO: Pass `--dry-run` when stabilized.
        log::warn!("Dry run: would have run `cargo install` for `{pkg_name}`.");
        log::info!("Bump verbosity if additional details are desired.");
        // TODO: Remove `Option` layer when passing `--dry-run`.
        Ok(None)
    } else {
        cmd.status()
            .map(Some)
            .wrap_err("Failed to execute Cargo.")
            .note(
                "This can happen for many reasons, but it should not happen easily at this point.",
            )
            .suggestion("Read the underlying error message.")
    }
}

/// Equivalent of [`install`] using `cargo-binstall` as a backend.
fn binstall(
    pkg_name: &str,
    pkg_req: &DetailedPackageReq,
    force: bool,
    dry_run: bool,
    verbosity: i8,
) -> Result<ExitStatus> {
    let mut cmd = Command::new(env_var()?);
    // The tool emits to stdout by default and does not have an option to
    // control that while only stderr is used here, so redirect instead.
    cmd.stdout(io::stderr());

    // Do the same regarding the custom environment variables and extra
    // arguments: the backends are exclusive for a package, so this should not
    // be too confusing, hopefully.
    if !pkg_req.environment.is_empty() {
        cmd.envs(&pkg_req.environment);
        log::trace!("Environment set: {:#?}", pkg_req.environment);
    }

    cmd.args([
        // What we want here.
        "binstall",
        // Telemetry is optional.
        "--disable-telemetry",
        // Non-interactive by design for now.
        "--no-confirm",
        // Also apply the verbosity.
        "--log-level",
        match verbosity {
            i8::MIN..=-3 => "off",
            -2 => "error",
            -1 => "warn",
            0 | 1 => "info",
            2 => "debug",
            3..=i8::MAX => "trace",
        },
        // The package version to install.
        "--version",
        &pkg_req.version.to_string(),
    ]);

    if let Some(index) = pkg_req.index.as_deref() {
        cmd.args(["--index", index]);
        log::trace!("`--index {}` args added.", index);
    }

    if let Some(registry) = pkg_req.registry.as_deref() {
        cmd.args(["--registry", registry]);
        log::trace!("`--registry {}` args added.", registry);
    }

    if let Some(git) = pkg_req.git.as_deref() {
        cmd.args(["--git", git]);
        log::trace!("`--git {}` args added.", git);
    }

    if force {
        cmd.arg("--force");
        log::trace!("`--force` arg added.");
    }

    if dry_run {
        cmd.arg("--dry-run");
        log::trace!("`--dry-run` arg added.");
        log::warn!("Dry run enabled in call to `cargo binstall` for `{pkg_name}`.");
        log::info!("Bump verbosity if additional details are desired.");
    }

    if pkg_req.locked {
        cmd.arg("--locked");
        log::trace!("`--locked` arg added.");
    }

    // This should be kept here: after all other options and before the `--`.
    if !pkg_req.extra_arguments.is_empty() {
        cmd.args(&pkg_req.extra_arguments);
        log::trace!("Extra arguments added: {:#?}", pkg_req.extra_arguments);
    }

    cmd.args(["--", pkg_name]);
    log_cmd(&cmd);

    cmd.status()
        .wrap_err("Failed to execute Cargo.")
        .note("This can happen for many reasons, but it should not happen easily at this point.")
        .suggestion("Read the underlying error message.")
}

/// Heuristically determines whether `cargo-binstall` is installed or not.
///
/// Currently, it first tries to see if it is among the installed packages if
/// they have been retrieved previously, and tries to call the program directly
/// with some additional validation if not.
fn binstall_is_available(installed: &BTreeSet<String>) -> bool {
    // When `--skip-check` is used.
    let res = if installed.is_empty() {
        binstall_version()
            .inspect(|ver| log::debug!("cargo-binstall successfully reports: {:?}", ver))
            .inspect_err(|err| log::debug!("cargo-binstall automatic detection failed: {:?}", err))
            .is_ok()
    } else {
        installed.contains("cargo-binstall")
    };

    log::debug!(
        "Considering cargo-binstall as {}available.",
        if res { "" } else { "not " },
    );
    res
}

/// Calls `cargo-binstall -V` and parses the returned version.
fn binstall_version() -> Result<Version> {
    let out = Command::new(env_var()?)
        .env_clear()
        .stdin(Stdio::null())
        .stderr(Stdio::null())
        .stdout(Stdio::piped())
        .args(["binstall", "-V"])
        .output()
        .wrap_err("Failed to execute Cargo.")
        .note("This can happen for many reasons, but it should not happen easily at this point.")
        .suggestion("Read the underlying error message.")?;

    if out.status.success() {
        String::from_utf8(out.stdout)
            .wrap_err("`cargo binstall -V` succeeded but returned a non-UTF8 output.")
            .note("This is rather unexpected.")
            .suggestion("Run it yourself to see if it behaves correctly.")?
            .trim_end()
            .parse()
            .wrap_err("`cargo binstall -V` succeeded but returned something else than a version.")
            .note("This is rather unexpected.")
            .suggestion("Run it yourself to see if it behaves correctly.")
    } else {
        Err(eyre!("Cargo or `cargo binstall` failed.")
            .note("This can simply be because it is not installed.")
            .suggestion("Install and run it yourself to see if it behaves correctly."))
    }
}

/// Runs `cargo install` or `binstall` for the given package depending on the
/// current Cargo-wise environment and specified choice strategy, with priority
/// for `binstall` when in auto mode.
#[expect(clippy::too_many_arguments, reason = "Plumbing.")]
fn install_one(
    installed: &BTreeSet<String>,
    pkg_name: &str,
    pkg_req: &DetailedPackageReq,
    force: bool,
    dry_run: bool,
    binstall: BinstallChoice,
    color: ColorChoice,
    verbosity: i8,
) -> Result<Option<ExitStatus>> {
    // HACK: disable the auto mode under testing to easily avoid bothersome
    // hacks and maintenance there in order to avoid failing when the tool is
    // installed locally compared to current CI. Tests targeting binstall have
    // to explicitly enable the feature through the `always` setting.
    if binstall == BinstallChoice::Always
        || binstall == BinstallChoice::Auto
            && !context_seems_testing()
            && binstall_is_available(installed)
    {
        log::debug!("Using `cargo-binstall` as the installation method.");
        self::binstall(pkg_name, pkg_req, force, dry_run, verbosity).map(Some)
    } else {
        log::debug!("Using `cargo install` as the installation method.");
        install(pkg_name, pkg_req, force, dry_run, color, verbosity)
    }
}

/// Runs `cargo install` or `binstall` for all packages listed in the given
/// user configuration and returns a per-package installation report.
///
/// Returns `Ok(report)` when `no_fail_fast` is `true`, otherwise `Err(err)` of
/// the first error `err` encountered.
#[expect(clippy::too_many_arguments, reason = "Plumbing.")]
pub fn install_all(
    packages: &BTreeMap<String, DetailedPackageReq>,
    installed: &BTreeSet<String>,
    no_fail_fast: bool,
    force: bool,
    dry_run: bool,
    binstall: BinstallChoice,
    color: ColorChoice,
    verbosity: i8,
) -> Result<InstallReport> {
    // Returned installation report.
    let mut rep = BTreeMap::new();
    // Aggregation of errors when `no_fail_fast` is enabled.
    let mut err_rep = None::<eyre::Report>;

    for (pkg_name, pkg) in packages {
        let is_installed = installed.contains(pkg_name);
        log::info!(
            "{}ing `{pkg_name}`...",
            if is_installed { "Updat" } else { "Install" }
        );

        if let Err(err) = install_one(
            installed,
            pkg_name,
            pkg,
            force || pkg.force,
            dry_run,
            // FIXME: this lets the per-package configuration have precedence
            // over the global defaults, but over the CLI as well; optionals
            // should be introduced in order to re-order things properly instead.
            pkg.binstall.unwrap_or(binstall),
            color,
            verbosity,
        )
        .and_then(|status| {
            status.map_or(Ok(()), |status| {
                status.success().then_some(()).ok_or_else(|| {
                    let err = eyre!("Cargo process finished unsuccessfully: {status}")
                        .note("This can happen for many reasons.")
                        .suggestion("Read Cargo's output.");

                    if no_fail_fast || pkg.no_fail_fast {
                        err
                    } else {
                        err.suggestion(
                            "Use `ship --no-fail-fast` to ignore this \
                            and continue on with other packages.",
                        )
                    }
                })
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
            if no_fail_fast || pkg.no_fail_fast {
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
    // HACK: detect the test context in order to adapt the execution to it.
    let is_test = context_seems_testing();
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
pub fn search_exact_all(
    pkgs: &BTreeMap<String, DetailedPackageReq>,
) -> Result<BTreeMap<String, Version>> {
    log::info!("Fetching latest package versions...");
    let mut procs = Vec::new();
    let mut vers = BTreeMap::new();

    log::debug!("Spawning search child processes in parallel...");
    for pkg in pkgs
        .iter()
        .filter_map(|(pkg_name, pkg_req)| (!pkg_req.skip_check).then_some(pkg_name))
    {
        procs.push(
            spawn_search_exact(pkg)
                .wrap_err_with(|| format!("Failed to spawn search for {pkg:?}."))?,
        );
    }

    log::debug!("Waiting for each search child processes to finish...");
    // Key traversal order is stable because sorted.
    for (pkg, proc) in pkgs
        .iter()
        .filter_map(|(pkg_name, pkg_req)| (!pkg_req.skip_check).then_some(pkg_name))
        .zip(procs.into_iter())
    {
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

/// Heuristically detects whether the current execution context seems like a
/// testing one or not.
///
/// It currently uses some environment variable that `cargo-test-support`'s
/// `cargo_test` macro sets for invoked Cargo instances.
fn context_seems_testing() -> bool {
    env::var_os("__CARGO_TEST_ROOT").is_some_and(|var| {
        let path = PathBuf::from(var);
        path.is_absolute() && path.is_dir()
    })
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
    use std::sync::{LazyLock, Mutex};

    use cargo_test_macro::cargo_test;

    use super::*;
    use crate::config::PackageRequirement;
    use crate::testing;

    const SELF: &str = clap::crate_name!();
    const NONE: &str = "azertyuiop-qsdfghjklm_wxcvbn";
    static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

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
                .map(|pkg| (pkg.to_owned(), PackageRequirement::SIMPLE_STAR.into()))
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
            &[(NONE.to_owned(), PackageRequirement::SIMPLE_STAR.into())]
                .into_iter()
                .collect()
        )
        .is_err());
    }

    #[ignore = "Long and online test."]
    #[cargo_test]
    fn test_singlethreaded_binstall() {
        let _lk = LOCK.lock();
        let _reg = testing::init_registry();
        testing::fake_install_all([
            (SELF, clap::crate_version!(), false),
            ("bat", "0.23.0", false),
        ]);
        testing::fake_publish_all([(SELF, clap::crate_version!()), ("bat", "0.24.0")]);
        testing::set_env();

        binstall(
            "bat",
            &DetailedPackageReq {
                version: "0.24.0".parse().unwrap(),
                ..Default::default()
            },
            false,
            false,
            0,
        )
        .unwrap();
    }

    #[cargo_test]
    fn test_singlethreaded_binstallisavailable_yes() {
        let _lk = LOCK.lock();
        testing::fake_install("cargo-binstall", "0.0.0", false);
        testing::set_env();

        assert!(binstall_is_available(
            &["cargo-binstall".to_owned()].into_iter().collect()
        ));
    }

    #[ignore = "Fails if not actually installed."]
    #[cargo_test]
    fn test_singlethreaded_binstallisavailable_emptyset_yes() {
        let _lk = LOCK.lock();
        testing::fake_install("cargo-binstall", "0.0.0", false);
        testing::set_env();

        assert!(binstall_is_available(&BTreeSet::new()));
    }

    #[cargo_test]
    fn test_singlethreaded_binstallisavailable_no() {
        let _lk = LOCK.lock();
        testing::fake_install("abc", "0.0.0", false);
        testing::set_env();

        assert!(!binstall_is_available(
            &["abc".to_owned()].into_iter().collect()
        ));
    }

    #[ignore = "Fails if actually installed."]
    #[cargo_test]
    fn test_singlethreaded_binstallisavailable_emptyset_no() {
        let _lk = LOCK.lock();
        testing::fake_install("abc", "0.0.0", false);
        testing::set_env();

        assert!(!binstall_is_available(&BTreeSet::new()));
    }

    #[ignore = "Fails if not actually installed in the precise version."]
    #[cargo_test]
    fn test_singlethreaded_binstallversion() {
        let _lk = LOCK.lock();
        testing::fake_install("cargo-binstall", "1.10.17", false);
        testing::set_env();

        assert_eq!(binstall_version().unwrap(), "1.10.17".parse().unwrap());
    }

    #[test]
    fn test_configget_withenv_installroot() -> Result<()> {
        let _lk = LOCK.lock();
        // SAFETY: mutually-exclusive test execution.
        unsafe { env::set_var("CARGO_INSTALL_ROOT", "/tmp") };
        assert_eq!(config_get("install.root")?, "/tmp");
        // SAFETY: mutually-exclusive test execution.
        unsafe { env::remove_var("CARGO_INSTALL_ROOT") };
        Ok(())
    }
}

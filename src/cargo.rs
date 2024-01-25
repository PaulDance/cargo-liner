//! Module handling the execution of cargo for various operations.
//!
//! See [`install_all`] in order to install configured packages.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::os::unix::prelude::OsStrExt;
use std::process::{Child, Command, Stdio};
use std::str;

use anyhow::{anyhow, Result};
use clap::ColorChoice;
use regex::Regex;
use semver::Version;

use crate::config::Package;

/// Installs a package, by running `cargo install` passing the `name`, `version` and requested
/// `features`.
///
/// The launched process' path is determined using the `$CARGO` environment
/// variable as it is set by Cargo when it calls an external subcommand's
/// corresponding program. See the [Cargo reference] for more details.
///
/// [Cargo reference]: https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands
fn install(
    name: &str,
    version: &str,
    no_default_features: bool,
    all_features: bool,
    features: &[String],
    force: bool,
    color: ColorChoice,
) -> Result<()> {
    let mut cmd = Command::new(env::var("CARGO")?);
    cmd.args([
        "--color",
        &color.to_string(),
        "install",
        "--version",
        version,
    ]);

    if no_default_features {
        cmd.arg("--no-default-features");
        trace!("`--no-default-features` arg added.");
    }

    if all_features {
        cmd.arg("--all-features");
        trace!("`--all-features` arg added.");
    }

    if !features.is_empty() {
        cmd.arg("--features").arg(features.join(","));
        trace!("`--features` arg added.");
    }

    if force {
        cmd.arg("--force");
        trace!("`--force` arg added.");
    }

    cmd.args(["--", name]);
    log_cmd(&cmd);

    cmd.status()?;
    Ok(())
}

/// Runs `cargo install` for all packages listed in the given user configuration.
pub fn install_all(
    packages: &BTreeMap<String, Package>,
    installed: &BTreeSet<String>,
    force: bool,
    color: ColorChoice,
) -> Result<()> {
    for (pkg_name, pkg) in packages {
        if installed.contains(pkg_name) {
            info!("Updating `{}`...", pkg_name);
        } else {
            info!("Installing `{}`...", pkg_name);
        }

        install(
            pkg_name,
            &pkg.version().to_string(),
            !pkg.default_features(),
            pkg.all_features(),
            pkg.features(),
            force,
            color,
        )?;
    }

    Ok(())
}

/// Spawns `cargo search` for the given package with only stdout piped and
/// returns the corresponding child process handle to be used with
/// [`finish_search_exact`].
fn spawn_search_exact(pkg: &str) -> Result<Child> {
    let mut cmd = Command::new(env::var("CARGO")?);

    cmd.stdin(Stdio::null());
    cmd.stderr(Stdio::null());
    cmd.stdout(Stdio::piped());
    cmd.args(["--color=never", "search"]);

    // HACK: detect test environment using some environment variable.
    if env::var_os("__CARGO_TEST_ROOT").is_some() {
        cmd.arg("--registry=dummy-registry");
    }

    cmd.args(["--limit=1", "--", pkg]);

    log_cmd(&cmd);
    Ok(cmd.spawn()?)
}

/// Waits for the given child process as spawned by [`spawn_search_exact`] to
/// finish and extract the received package version from the output.
fn finish_search_exact(pkg: &str, proc: Child) -> Result<Version> {
    let out = str::from_utf8(proc.wait_with_output()?.stdout.as_slice())?.to_owned();
    trace!("Search for {:#?} got: {:#?}", pkg, out);

    // See https://semver.org/#backusnaur-form-grammar-for-valid-semver-versions.
    let ver = Regex::new(&format!(r#"{pkg}\s=\s"([0-9a-zA-Z.+-]+)"\s+#.*"#))?
        .captures(out.lines().next().ok_or_else(|| {
            anyhow!("Not at least one line in search output for {pkg:#?}: does the package exist?")
        })?)
        .ok_or_else(|| {
            anyhow!(
                "No regex capture while parsing search output for {pkg:#?}: does the package exist?"
            )
        })?
        .get(1)
        .ok_or_else(|| {
            anyhow!("Version not captured by regex matching search output for {pkg:#?}.")
        })?
        .as_str()
        .parse::<Version>()?;
    trace!("Parsed version is: {:#?}.", ver);

    Ok(ver)
}

/// Runs `*_search_exact` for all packages in the given map and returns the
/// thus fetched versions in the collected map.
pub fn search_exact_all(pkgs: &BTreeMap<String, Package>) -> Result<BTreeMap<String, Version>> {
    info!("Fetching latest package versions...");
    let mut procs = Vec::new();
    let mut vers = BTreeMap::new();

    debug!("Spawning search child processes in parallel...");
    for pkg in pkgs.keys() {
        procs.push(spawn_search_exact(pkg)?);
    }

    debug!("Waiting for each search child processes to finish...");
    // Key traversal order is stable because sorted.
    for (pkg, proc) in pkgs.keys().zip(procs.into_iter()) {
        vers.insert(pkg.clone(), finish_search_exact(pkg, proc)?);
    }

    Ok(vers)
}

/// Runs `cargo config get` with the given configuration key and returns the
/// collected string value.
pub fn config_get(key: &str) -> Result<String> {
    let mut cmd = Command::new(env::var("CARGO")?);
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
    let out = cmd.output()?;
    out.status.success().then_some(()).ok_or_else(|| {
        anyhow!(
            "Command failed with status: {:#?} and stderr: {:#?}.",
            out.status,
            OsStr::from_bytes(&out.stderr),
        )
    })?;

    let out_str = OsStr::from_bytes(&out.stdout);
    trace!("Got: {out_str:#?}.");
    Ok(out_str
        .to_string_lossy()
        .trim_end()
        .trim_matches('"')
        .to_owned())
}

/// Logs the program and arguments of the given command to DEBUG.
fn log_cmd(cmd: &Command) {
    debug!(
        "Running {:#?} with arguments {:#?}...",
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<_>>(),
    );
}

#[cfg(test)]
mod tests {
    use anyhow::bail;
    use cargo_test_macro::cargo_test;

    use super::*;
    use crate::testing;

    const SELF: &str = clap::crate_name!();
    const NONE: &str = "azertyuiop-qsdfghjklm_wxcvbn";

    #[cargo_test]
    fn test_searchspawn_self_isok() -> Result<()> {
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
        assert!(str::from_utf8(res.stdout.as_slice())?.lines().count() > 0);

        Ok(())
    }

    #[cargo_test]
    fn test_searchfinish_self_isok() -> Result<()> {
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
    fn test_searchspawn_none_isok() -> Result<()> {
        let _reg = testing::init_registry();
        testing::set_env();

        let proc = spawn_search_exact(NONE)?;
        assert_ne!(proc.id(), 0);
        assert!(proc.stdin.is_none());
        assert!(proc.stderr.is_none());
        assert!(proc.stdout.is_some());

        let res = proc.wait_with_output()?;
        assert!(res.status.success());
        assert_eq!(str::from_utf8(res.stdout.as_slice())?.lines().count(), 0);

        Ok(())
    }

    #[cargo_test]
    fn test_searchfinish_none_iserr() -> Result<()> {
        let _reg = testing::init_registry();
        testing::set_env();

        assert!(finish_search_exact(NONE, spawn_search_exact(NONE)?).is_err());
        Ok(())
    }

    #[cargo_test]
    fn test_searchall_selfandothers_isok() -> Result<()> {
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
                    pkg => bail!("Unexpected package: {pkg:?}"),
                }
                .parse()?,
            );
        }
        Ok(())
    }

    #[cargo_test]
    fn test_searchall_none_iserr() {
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

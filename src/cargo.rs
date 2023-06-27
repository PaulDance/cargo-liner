//! Module handling the execution of cargo for various operations.
//!
//! See [`install_all`] in order to install configured packages.

use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::ffi::OsStr;
use std::process::{Child, Command, Stdio};
use std::str;

use anyhow::{anyhow, Result};
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
) -> Result<()> {
    let mut cmd = Command::new(env::var("CARGO")?);
    cmd.args(["install", "--version", version, name]);

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

    log_cmd(&cmd);
    cmd.status()?;
    Ok(())
}

/// Runs `cargo install` for all packages listed in the given user configuration.
pub fn install_all(
    packages: &BTreeMap<String, Package>,
    installed: &BTreeSet<String>,
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
    cmd.args(["search", "--limit=1", pkg]);
    log_cmd(&cmd);
    Ok(cmd.spawn()?)
}

/// Waits for the given child process as spawned by [`spawn_search_exact`] to
/// finish and extract the received package version from the output.
fn finish_search_exact(pkg: &str, proc: Child) -> Result<Version> {
    let out = str::from_utf8(proc.wait_with_output()?.stdout.as_slice())?.to_owned();
    trace!("Search for {:?} got: {:?}", pkg, out);

    let ver = Regex::new(&format!(r#"{pkg}\s=\s"([0-9.abrc]+)"\s+#.*"#))?
        .captures(
            out.lines()
                .next()
                .ok_or_else(|| anyhow!("Not at least one line in search output for {pkg:?}."))?,
        )
        .ok_or_else(|| anyhow!("No regex capture while parsing search output for {pkg:?}."))?
        .get(1)
        .ok_or_else(|| {
            anyhow!("Version not captured by regex matching search output for {pkg:?}.")
        })?
        .as_str()
        .parse::<Version>()?;
    trace!("Parsed version is: {:?}.", ver);

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

/// Logs the program and arguments of the given command to DEBUG.
fn log_cmd(cmd: &Command) {
    debug!(
        "Running {:?} with arguments {:?}...",
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<_>>(),
    );
}

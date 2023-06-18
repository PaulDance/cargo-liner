//! Module handling the execution of cargo for various operations.
//!
//! See [`install_all`] in order to install configured packages.

use std::collections::BTreeMap;
use std::env;
use std::ffi::OsStr;
use std::process::Command;

use anyhow::Result;

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

    debug!(
        "Running {:?} with arguments {:?}...",
        cmd.get_program().to_string_lossy(),
        cmd.get_args()
            .map(OsStr::to_string_lossy)
            .collect::<Vec<_>>(),
    );
    cmd.status()?;
    Ok(())
}

/// Runs `cargo install` for all packages listed in the given user configuration.
pub fn install_all(packages: &BTreeMap<String, Package>) -> Result<()> {
    for (pkg_name, pkg) in packages {
        info!("Installing or updating `{}`...", pkg_name);
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

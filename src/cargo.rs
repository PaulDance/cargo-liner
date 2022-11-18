//! Module handling the execution of cargo for various operations.
//!
//! See [`install_all`] in order to install configured packages.

use std::collections::BTreeMap;
use std::env;
use std::process::Command;

use anyhow::Result;

use crate::config::Package;

/// Runs `cargo install` for a package from its given `name` and `version`.
///
/// The launched process' path is determined using the `$CARGO` environment
/// variable as it is set by Cargo when it calls an external subcommand's
/// corresponding program. See the [Cargo reference] for more details.
///
/// [Cargo reference]: https://doc.rust-lang.org/cargo/reference/external-tools.html#custom-subcommands
fn install(name: &str, version: &str) -> Result<()> {
    Command::new(env::var("CARGO")?)
        .args(["install", "--version", version, name])
        .status()?;
    Ok(())
}

/// Runs `cargo install` for all packages listed in the given user configuration.
pub fn install_all(packages: &BTreeMap<String, Package>) -> Result<()> {
    for (name, value) in packages {
        match value {
            Package::Simple(version) => install(name, &version.to_string())?,
        }
    }
    Ok(())
}

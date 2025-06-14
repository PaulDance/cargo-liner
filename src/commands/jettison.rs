use std::collections::{BTreeMap, BTreeSet};

use color_eyre::Result;
use color_eyre::eyre::Context;

use crate::cargo;
use crate::cli::JettisonArgs;
use crate::config::{CargoCratesToml, PackageRequirement, UserConfig};

pub fn run(_args: &JettisonArgs, cargo_verbosity: i8) -> Result<()> {
    let config = UserConfig::parse_file().wrap_err("Failed to parse the user configuration.")?;
    let cct =
        CargoCratesToml::parse_file().wrap_err("Failed to parse Cargo's .crates.toml file.")?;

    cargo::uninstall_all(
        needing_uninstall(cct.into_names(), &config.packages),
        cargo_verbosity,
    )
    .wrap_err("Some package failed to uninstall.")?;
    Ok(())
}

/// Computes the package names that need uninstallation: all those installed
/// but not part of the user-configured ones.
fn needing_uninstall(
    mut installed: BTreeSet<String>,
    configured: &BTreeMap<String, PackageRequirement>,
) -> impl Iterator<Item = String> {
    // Always exclude self from uninstallation.
    installed.remove(clap::crate_name!());
    installed
        .into_iter()
        .filter(|pkg| !configured.contains_key(pkg))
}

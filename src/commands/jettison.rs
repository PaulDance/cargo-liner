use std::collections::BTreeMap;

use clap::ColorChoice;
use color_eyre::Result;
use color_eyre::eyre::Context;
use semver::Version;

use crate::cargo;
use crate::cli::JettisonArgs;
use crate::config::{CargoCratesToml, PackageRequirement, UserConfig};

pub fn run(_args: &JettisonArgs, cargo_color: ColorChoice, cargo_verbosity: i8) -> Result<()> {
    let config = UserConfig::parse_file().wrap_err("Failed to parse the user configuration.")?;
    let cct =
        CargoCratesToml::parse_file().wrap_err("Failed to parse Cargo's .crates.toml file.")?;

    let to_uninstall = needing_uninstall(cct.into_name_versions(), &config.packages);
    cargo::uninstall_all(to_uninstall.into_keys(), cargo_color, cargo_verbosity)
        .wrap_err("Some package failed to uninstall.")?;
    Ok(())
}

/// Computes the package names that need uninstallation: all those installed
/// but not part of the user-configured ones.
fn needing_uninstall(
    mut installed: BTreeMap<String, Version>,
    configured: &BTreeMap<String, PackageRequirement>,
) -> BTreeMap<String, Version> {
    // Always exclude self from uninstallation.
    installed.remove(clap::crate_name!());
    installed
        .into_iter()
        .filter(|(pkg, _)| !configured.contains_key(pkg))
        .collect()
}

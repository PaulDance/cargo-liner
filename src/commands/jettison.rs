use std::collections::BTreeMap;

use clap::ColorChoice;
use color_eyre::Result;
use color_eyre::eyre::Context;
use semver::Version;
use tabled::Tabled;

use crate::cargo;
use crate::cli::JettisonArgs;
use crate::commands::styled_table;
use crate::config::{CargoCratesToml, PackageRequirement, UserConfig};

pub fn run(_args: &JettisonArgs, cargo_color: ColorChoice, cargo_verbosity: i8) -> Result<()> {
    let config = UserConfig::parse_file().wrap_err("Failed to parse the user configuration.")?;
    let cct =
        CargoCratesToml::parse_file().wrap_err("Failed to parse Cargo's .crates.toml file.")?;

    let to_uninstall = needing_uninstall(cct.into_name_versions(), &config.packages);
    log_uninstallation_plan(&to_uninstall);

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
    log::debug!("Computing uninstallation plan...");
    // Always exclude self from uninstallation.
    installed.remove(clap::crate_name!());
    installed
        .into_iter()
        .filter(|(pkg, _)| !configured.contains_key(pkg))
        .collect()
}

/// Logs the packages that will be removed.
fn log_uninstallation_plan(to_uninstall: &BTreeMap<String, Version>) {
    if to_uninstall.is_empty() {
        log::info!("No package to uninstall: all installed are configured as well.");
    } else {
        log::info!(
            "Will uninstall:\n{}",
            styled_table(to_uninstall.iter().map(|(pkg, ver)| PackageEntry {
                name: pkg.clone(),
                version: ver.to_string(),
            })),
        );
    }
}

/// [`Tabled`] for logging packages.
#[derive(Tabled)]
struct PackageEntry {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Version")]
    version: String,
}

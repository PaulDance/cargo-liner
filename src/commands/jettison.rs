use std::collections::BTreeMap;
use std::io::{self, BufRead};

use clap::ColorChoice;
use color_eyre::eyre::Context;
use color_eyre::{Result, Section};
use semver::Version;
use tabled::Tabled;

use crate::cargo;
use crate::cli::JettisonArgs;
use crate::commands::styled_table;
use crate::config::{CargoCratesToml, PackageRequirement, UserConfig};

pub fn run(args: &JettisonArgs, cargo_color: ColorChoice, cargo_verbosity: i8) -> Result<()> {
    let config = UserConfig::parse_file().wrap_err("Failed to parse the user configuration.")?;
    let cct =
        CargoCratesToml::parse_file().wrap_err("Failed to parse Cargo's .crates.toml file.")?;

    let to_uninstall = needing_uninstall(cct.into_name_versions(), &config.packages);
    log_uninstallation_plan(&to_uninstall);

    // Nothing to do in this case anyway, so cut short.
    if to_uninstall.is_empty() {
        return Ok(());
    }

    if args.no_confirm || args.dry_run {
        log::warn!(
            "Skipping interactive confirmation, as requested with `{}`.",
            if args.no_confirm {
                "--no-confirm"
            } else {
                "--dry-run"
            },
        );
    } else if ask_confirmation().wrap_err("Failed to ask for interactive confirmation.")? {
        log::info!("Aborting.");
        return Ok(());
    }

    cargo::uninstall_all(
        to_uninstall.into_keys(),
        args.no_fail_fast,
        args.dry_run,
        cargo_color,
        cargo_verbosity,
    )
    .wrap_err_with(|| {
        format!(
            "{} package failed to uninstall.",
            if args.no_fail_fast {
                "At least one"
            } else {
                "Some"
            }
        )
    })?;
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

/// Interactively asks for user confirmation of this operation.
///
/// Returns whether the operation should be aborted or not, defaulting to not.
fn ask_confirmation() -> Result<bool> {
    log_confirmation();
    let stdin = io::stdin().lock();

    for line in stdin.lines() {
        let line = line
            .wrap_err("Failed to read from stdin.")
            .note("This shouldn't happen easily at this point.")
            .suggestion("Read the underlying error message.")?;

        if line.is_empty() || line == "y" {
            return Ok(false);
        } else if line == "n" {
            return Ok(true);
        }

        log_confirmation();
    }

    Ok(false)
}

/// Question used in [`ask_confirmation`].
#[inline]
fn log_confirmation() {
    log::warn!("This will remove all the packages listed above: do you confirm? [Y/n]");
}

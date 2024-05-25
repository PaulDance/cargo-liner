//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
#![warn(unused_crate_dependencies)]

use std::collections::{BTreeMap, BTreeSet};
use std::{env, process};

use clap::ColorChoice;
use color_eyre::config::{HookBuilder, Theme};
use color_eyre::eyre::{self, Result, WrapErr};
use color_eyre::Section;
use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;
use semver::Version;
use tabled::settings::Style;
use tabled::{Table, Tabled};
#[cfg(test)]
use tempfile as _;
#[cfg(test)]
use trycmd as _;

mod cargo;
mod cli;
use cli::{LinerArgs, LinerCommands};
mod config;
use config::{CargoCratesToml, Package, UserConfig};
#[cfg(test)]
#[path = "../tests/common/mod.rs"]
mod testing;

pub const OPEN_ISSUE_MSG: &str =
    "Open an issue using https://github.com/PaulDance/cargo-liner/issues/new/choose.";

/// Wrap the desired main and let `color-eyre` display errors.
fn main() -> Result<()> {
    // Logging and error reporting are controlled by the CLI arguments, so they
    // must be parsed first.
    let args = LinerArgs::parse_env();

    // Let the CLI verbosity have control over the backtraces displayed.
    if let verb @ 1.. = args.verbosity() {
        env::set_var(
            "RUST_BACKTRACE",
            match verb {
                1 => "1",
                _ => "full",
            },
        );
    }

    // Explicitely disable error colors when explicitely asked to do so.
    if args.color == ColorChoice::Never {
        HookBuilder::new().theme(Theme::new()).install()
    } else {
        color_eyre::install()
    }
    .wrap_err("Failed to install the error hook.")
    .note("This should only happen if it is attempted twice.")
    .suggestion(OPEN_ISSUE_MSG)?;

    // HACK: reproduce the previous behavior by directly exiting: don't display
    // anything, but only report an error code when verbosity is low enough.
    try_main(&args).inspect_err(|_| {
        if args.verbosity() <= -3 {
            process::exit(1);
        }
    })
}

/// Actual main operation.
#[allow(clippy::too_many_lines)]
fn try_main(args: &LinerArgs) -> Result<()> {
    let mut bld = pretty_env_logger::formatted_builder();
    bld.parse_default_env();

    // Logging setup parameterized by CLI args.
    if args.verbosity().abs() < 2 {
        bld.filter_module("::", LevelFilter::Error);
        bld.filter_module(
            env!("CARGO_CRATE_NAME"),
            match args.verbosity() {
                0 => LevelFilter::Info,
                1 => LevelFilter::Debug,
                -1 => LevelFilter::Warn,
                _ => unreachable!(),
            },
        );
    } else {
        bld.filter_level(match args.verbosity() {
            2 => LevelFilter::Debug,
            v if v > 2 => LevelFilter::Trace,
            -2 => LevelFilter::Error,
            q if q < -2 => LevelFilter::Off,
            _ => unreachable!(),
        });
    }
    bld.write_style(match args.color {
        ColorChoice::Always => WriteStyle::Always,
        ColorChoice::Auto => WriteStyle::Auto,
        ColorChoice::Never => WriteStyle::Never,
    });
    bld.try_init()
        .wrap_err("Failed to initialize the logger.")
        .note("This should only happen if it is attempted twice.")
        .suggestion(OPEN_ISSUE_MSG)?;

    // CLI command dispatch.
    match &args.command {
        Some(LinerCommands::Import(import_args)) => {
            if UserConfig::file_path()
                .wrap_err("Failed to build the configuration file path.")?
                .try_exists()
                .wrap_err("Failed to check if the configuration file exists.")
                .suggestion("Check the permissions of the Cargo home directory.")?
            {
                if import_args.force {
                    log::warn!("Configuration file will be overwritten.");
                } else {
                    eyre::bail!("Configuration file already exists, use -f/--force to overwrite.");
                }
            }
            log::info!("Importing Cargo installed crates as a new configuration file...");
            // Clap conflict settings ensure the options are mutually exclusive.
            (if import_args.force {
                UserConfig::overwrite_file
            } else {
                UserConfig::save_file
            })(&(if import_args.exact {
                CargoCratesToml::into_exact_version_config
            } else if import_args.compatible {
                CargoCratesToml::into_comp_version_config
            } else if import_args.patch {
                CargoCratesToml::into_patch_version_config
            } else {
                CargoCratesToml::into_star_version_config
            })(
                CargoCratesToml::parse_file()
                    .wrap_err("Failed to parse Cargo's .crates.toml file.")?,
                import_args.keep_self,
                import_args.keep_local,
            ))
            .wrap_err("Failed to save the configuration file.")?;
        }
        cmd => {
            let mut skip_check = false;
            let mut force = false;
            let mut config =
                UserConfig::parse_file().wrap_err("Failed to parse the user configuration.")?;
            let cargo_verbosity = match args.verbosity() {
                -2..=1 => 0,
                v if v > 1 => v - 1,
                q if q < -2 => q + 2,
                _ => unreachable!(),
            };

            if let Some(LinerCommands::Ship(ship_args)) = cmd {
                skip_check = ship_args.skip_check;
                force = ship_args.force;
                config = config
                    .self_update(!ship_args.no_self)
                    .update_others(!ship_args.only_self);
            }

            if skip_check {
                // Don't parse `.crates.toml` here: can be used as a workaround.
                cargo::install_all(
                    &config.packages,
                    &BTreeSet::new(),
                    force,
                    args.color,
                    cargo_verbosity,
                )
                .wrap_err("Failed to install or update the configured packages.")?;
            } else {
                let cct = CargoCratesToml::parse_file()
                    .wrap_err("Failed to parse Cargo's .crates.toml file.")?;
                let vers = cargo::search_exact_all(&config.packages)
                    .wrap_err("Failed to fetch the latest versions of the configured packages.")?;
                log_summary(&vers, &cct.clone().into_name_versions());

                cargo::install_all(
                    &needing_install(&config.packages, &vers, &cct.clone().into_name_versions()),
                    &cct.into_names(),
                    force,
                    args.color,
                    cargo_verbosity,
                )
                .wrap_err("Failed to install or update the configured packages.")?;
            }
        }
    }

    log::info!("Done.");
    Ok(())
}

#[derive(Tabled)]
struct PackageStatus {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Status")]
    status: String,
}

/// Displays whether each package needs an update or not.
fn log_summary(new_vers: &BTreeMap<String, Version>, old_vers: &BTreeMap<String, Version>) {
    if new_vers.is_empty() {
        log::info!("No packages installed.");
    } else {
        log::info!(
            "Results:\n{}",
            Table::new(new_vers.iter().map(|(name, new_ver)| PackageStatus {
                name: name.clone(),
                status: old_vers.get(name).map_or_else(
                    || format!("ø -> {new_ver}"),
                    |old_ver| {
                        if old_ver < new_ver {
                            format!("{old_ver} -> {new_ver}")
                        } else {
                            format!("✔ {new_ver}")
                        }
                    },
                ),
            }))
            .with(Style::sharp())
        );
    }
}

/// Returns the packages that do indeed need an install or update.
fn needing_install(
    pkgs: &BTreeMap<String, Package>,
    new_vers: &BTreeMap<String, Version>,
    old_vers: &BTreeMap<String, Version>,
) -> BTreeMap<String, Package> {
    let mut to_install = BTreeMap::new();
    log::debug!("Filtering packages by versions...");

    for (pkg_name, pkg) in pkgs {
        if old_vers
            .get(pkg_name)
            .map_or(true, |ver| ver < new_vers.get(pkg_name).unwrap())
        {
            to_install.insert(pkg_name.clone(), pkg.clone());
            log::trace!("{pkg_name:?} is selected to be installed or updated.");
        } else {
            log::trace!("{pkg_name:?} is not selected: already up-to-date.");
        }
    }

    log::trace!("Filtered packages: {to_install:#?}.");
    to_install
}

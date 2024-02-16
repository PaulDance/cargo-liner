//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
use anyhow::{bail, Result};
use std::collections::{BTreeMap, BTreeSet};
use std::env;
use std::process::ExitCode;
#[macro_use]
extern crate log;
use clap::ColorChoice;
use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;
use semver::Version;
use tabled::settings::Style;
use tabled::{Table, Tabled};
mod cargo;
mod cli;
use cli::{LinerArgs, LinerCommands};
mod config;
use config::{CargoCratesToml, Package, UserConfig};

#[cfg(test)]
#[path = "../tests/common/mod.rs"]
mod testing;

/// Wrap the desired main and display errors in a fashion consistent with the
/// rest of the messages.
fn main() -> ExitCode {
    match try_main() {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            error!("{}", err);
            ExitCode::FAILURE
        }
    }
}

/// Actual main operation.
fn try_main() -> Result<()> {
    // Logging is controlled by args, so they must be parsed first.
    let args = LinerArgs::parse_env();
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
    bld.try_init()?;

    // CLI command dispatch.
    match &args.command {
        Some(LinerCommands::Import(import_args)) => {
            if UserConfig::file_path()?.try_exists()? {
                if import_args.force {
                    warn!("Configuration file will be overwritten.");
                } else {
                    bail!("Configuration file already exists, use -f/--force to overwrite.");
                }
            }
            info!("Importing Cargo installed crates as a new configuration file...");
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
                CargoCratesToml::parse_file()?,
                import_args.keep_self,
                import_args.with_local,
            ))?;
        }
        cmd => {
            let mut skip_check = false;
            let mut force = false;
            let mut config = UserConfig::parse_file()?;
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
                )?;
            } else {
                let cct = CargoCratesToml::parse_file()?;
                let vers = cargo::search_exact_all(&config.packages)?;
                log_summary(&vers, &cct.clone().into_name_versions());

                cargo::install_all(
                    &needing_install(&config.packages, &vers, &cct.clone().into_name_versions()),
                    &cct.into_names(),
                    force,
                    args.color,
                    cargo_verbosity,
                )?;
            }
        }
    }

    info!("Done.");
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
        info!("No packages installed.");
        return;
    }

    let pkgs = new_vers.iter().map(|(name, new_ver)| PackageStatus {
        name: name.to_string(),
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
    });

    info!("Results:\n{}", Table::new(pkgs).with(Style::sharp()));
}

/// Returns the packages that do indeed need an install or update.
fn needing_install(
    pkgs: &BTreeMap<String, Package>,
    new_vers: &BTreeMap<String, Version>,
    old_vers: &BTreeMap<String, Version>,
) -> BTreeMap<String, Package> {
    let mut to_install = BTreeMap::new();
    debug!("Filtering packages by versions...");

    for (pkg_name, pkg) in pkgs {
        if old_vers
            .get(pkg_name)
            .map_or(true, |ver| ver < new_vers.get(pkg_name).unwrap())
        {
            to_install.insert(pkg_name.clone(), pkg.clone());
            trace!("{:#?} is selected to be installed or updated.", pkg_name);
        } else {
            trace!("{:#?} is not selected: already up-to-date.", pkg_name);
        }
    }

    trace!("Filtered packages: {:#?}.", &to_install);
    to_install
}

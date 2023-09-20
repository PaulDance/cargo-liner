//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
use std::collections::{BTreeMap, BTreeSet};
use std::env;

use anyhow::{bail, Result};
#[macro_use]
extern crate log;
use log::LevelFilter;
use semver::Version;

mod cargo;
mod cli;
use cli::{LinerArgs, LinerCommands};
mod config;
use config::{CargoCratesToml, Package, UserConfig};

/// Wrap the desired main and display errors in a fashion consistent with the
/// rest of the messages.
fn main() {
    if let Err(err) = wrapped_main() {
        error!("{}", err);
    }
}

/// Actual main operation.
fn wrapped_main() -> Result<()> {
    // Logging is controlled by args, so they must be parsed first.
    let args = LinerArgs::parse_env();
    let mut bld = pretty_env_logger::formatted_builder();

    // Logging setup parameterized by CLI args.
    if args.verbose < 2 && args.quiet < 2 {
        bld.filter_module("::", LevelFilter::Error);
        bld.filter_module(
            env!("CARGO_CRATE_NAME"),
            if args.verbose == 0 && args.quiet == 0 {
                LevelFilter::Info
            } else if args.verbose == 1 {
                LevelFilter::Debug
            } else {
                LevelFilter::Warn
            },
        );
    } else {
        bld.filter_level(match (args.verbose, args.quiet) {
            (2, 0) => LevelFilter::Debug,
            // Three -v or more.
            (_, 0) => LevelFilter::Trace,
            (0, 2) => LevelFilter::Error,
            // Three -q or more.
            _ => LevelFilter::Off,
        });
    }
    bld.parse_default_env();
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
                CargoCratesToml::parse_file()?, import_args.keep_self
            ))?;
        }
        cmd => {
            let mut skip_check = false;
            let mut force = false;
            let mut config = UserConfig::parse_file()?;

            if let Some(LinerCommands::Ship(ship_args)) = cmd {
                skip_check = ship_args.skip_check;
                force = ship_args.force;
                config = config
                    .self_update(!ship_args.no_self)
                    .update_others(!ship_args.only_self);
            }

            if skip_check {
                // Don't parse `.crates.toml` here: can be used as a workaround.
                cargo::install_all(&config.packages, &BTreeSet::new(), force)?;
            } else {
                let cct = CargoCratesToml::parse_file()?;
                let vers = cargo::search_exact_all(&config.packages)?;
                log_summary(&config.packages, &vers, &cct.clone().into_name_versions());

                cargo::install_all(
                    &needing_install(&config.packages, &vers, &cct.clone().into_name_versions()),
                    &cct.into_names(),
                    force,
                )?;
            }
        }
    }

    info!("Done.");
    Ok(())
}

/// Displays whether each package needs an update or not.
fn log_summary(
    pkgs: &BTreeMap<String, Package>,
    new_vers: &BTreeMap<String, Version>,
    old_vers: &BTreeMap<String, Version>,
) {
    if let Some(max_len) = pkgs.keys().map(String::len).max() {
        info!("Results:");

        for pkg in pkgs.keys() {
            let new_ver = new_vers.get(pkg).unwrap();
            info!(
                "    {:<max_len$}  {}",
                pkg,
                old_vers.get(pkg).map_or_else(
                    || format!("ø -> {new_ver}"),
                    |old_ver| if old_ver < new_ver {
                        format!("{old_ver} -> {new_ver}")
                    } else {
                        "✔".to_owned()
                    }
                ),
            );
        }
    }
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
            trace!("{:?} is selected to be installed or updated.", pkg_name);
        } else {
            trace!("{:?} is not selected: already up-to-date.", pkg_name);
        }
    }

    trace!("Filtered packages: {:?}.", &to_install);
    to_install
}

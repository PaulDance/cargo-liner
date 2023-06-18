//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
use std::env;

use anyhow::{bail, Result};
#[macro_use]
extern crate log;
use log::LevelFilter;

mod cargo;
mod cli;
use cli::{LinerArgs, LinerCommands};
mod config;
use config::{CargoCratesToml, UserConfig};

/// Wrap the desired main and display errors in a fashion consistent with the
/// rest of the messages.
fn main() {
    if let Err(err) = wrapped_main() {
        error!("{}", err);
    }
}

/// Actual main operation.
fn wrapped_main() -> Result<()> {
    let args = LinerArgs::parse_env();
    let mut bld = pretty_env_logger::formatted_builder();
    // Use the INFO log level only for this crate by default.
    bld.filter_module("::", LevelFilter::Error);
    bld.filter_module(env!("CARGO_CRATE_NAME"), LevelFilter::Info);
    bld.parse_default_env();
    bld.try_init()?;

    match &args.command {
        Some(LinerCommands::Import(import_args)) => {
            if !import_args.force && UserConfig::file_path()?.try_exists()? {
                bail!("Configuration file already exists, use -f/--force to overwrite.");
            }
            // Clap conflict settings ensure the options are mutually exclusive.
            (if import_args.exact {
                CargoCratesToml::into_exact_version_config
            } else if import_args.compatible {
                CargoCratesToml::into_comp_version_config
            } else if import_args.patch {
                CargoCratesToml::into_patch_version_config
            } else {
                CargoCratesToml::into_star_version_config
            })(CargoCratesToml::parse_file()?, import_args.keep_self)
            .save_file()?;
        }
        Some(LinerCommands::Ship(ship_args)) => {
            let config = UserConfig::parse_file()?
                .self_update(!ship_args.no_self)
                .update_others(!ship_args.only_self);
            cargo::install_all(&config.packages)?;
        }
        None => cargo::install_all(&UserConfig::parse_file()?.packages)?,
    }

    Ok(())
}

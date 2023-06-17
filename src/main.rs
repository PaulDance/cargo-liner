//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
use anyhow::{bail, Result};

mod cargo;
mod cli;
use cli::{LinerArgs, LinerCommands};
mod config;
use config::{CargoCratesToml, UserConfig};

fn main() -> Result<()> {
    let args = LinerArgs::parse_env();

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
            })(CargoCratesToml::parse_file()?)
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

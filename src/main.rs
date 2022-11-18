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
            CargoCratesToml::parse_file()?
                .into_versionless_config()
                .save_file()?;
        }
        Some(LinerCommands::Ship) | None => {
            let config = UserConfig::parse_file()?;
            cargo::install_all(&config)?;
        }
    }

    Ok(())
}

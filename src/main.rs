//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
use anyhow::{bail, Result};
use home::cargo_home;

mod cargo;
mod cli;
use cli::{LinerArgs, LinerCommands};
mod config;
use config::{CargoCratesToml, UserConfig};

/// The default name for the configuration file in Cargo's home.
pub(crate) static CONFIG_FILE_NAME: &str = "liner.toml";

fn main() -> Result<()> {
    let args = LinerArgs::parse_env();

    match &args.command {
        Some(LinerCommands::Import(import_args)) => {
            if !import_args.force && cargo_home()?.join(CONFIG_FILE_NAME).try_exists()? {
                bail!("Configuration file already exists, use -f/--force to overwrite.");
            }
            CargoCratesToml::parse_file()?
                .into_versionless_config()
                .save_file()?;
        }
        _ => {
            let config = UserConfig::parse_file()?;
            cargo::install_all(&config)?;
        }
    }

    Ok(())
}

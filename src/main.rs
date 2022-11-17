//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.

use anyhow::Result;

mod cargo;
mod cli;
use cli::LinerArgs;
mod config;
use config::UserConfig;

/// The default name for the configuration file in Cargo's home.
pub(crate) static CONFIG_FILE_NAME: &str = "liner.toml";

fn main() -> Result<()> {
    // Output unused for now, just validates the input.
    let _args = LinerArgs::parse_env();
    let config = UserConfig::parse_file()?;
    cargo::install_all(&config)?;
    Ok(())
}

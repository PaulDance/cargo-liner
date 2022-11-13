use anyhow::Result;

mod cargo;
mod cli;
mod config;

pub(crate) static CONFIG_FILE_NAME: &str = "liner.toml";

fn main() -> Result<()> {
    let _args = cli::parse_args();
    let config = config::parse_config()?;
    cargo::install_all(&config)?;
    Ok(())
}

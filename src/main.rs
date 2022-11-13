use anyhow::Result;

mod cli;
mod config;

pub(crate) static CONFIG_FILE_NAME: &str = "liner.toml";

fn main() -> Result<()> {
    dbg!(cli::parse_args());
    dbg!(config::parse_config())?;
    Ok(())
}

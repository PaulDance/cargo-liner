use anyhow::Result;

mod cli;
mod config;

fn main() -> Result<()> {
    dbg!(cli::parse_args());
    dbg!(config::parse_config())?;
    Ok(())
}

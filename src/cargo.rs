use std::env;
use std::process::Command;

use anyhow::Result;

use crate::config::{Package, UserConfig};

fn install(name: &str, version: &str) -> Result<()> {
    Command::new(env::var("CARGO")?)
        .args(["install", "--version", version, name])
        .status()?;
    Ok(())
}

pub fn install_all(config: &UserConfig) -> Result<()> {
    for (name, value) in &config.packages {
        match value {
            Package::Simple(version) => install(name, &version.to_string())?,
        }
    }
    Ok(())
}

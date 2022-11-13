use std::env;
use std::process::Command;

use anyhow::Result;

use crate::config::{Package, UserConfig};

fn install<'a>(name: &'a str, version: &'a str) -> Result<()> {
    Command::new(env::var("CARGO")?)
        .args(["install", "--version", version, name])
        .status()?;
    Ok(())
}

pub fn install_all<'a>(config: &'a UserConfig) -> Result<()> {
    for (name, value) in config.packages.iter() {
        match value {
            Package::Simple(version) => install(&name, &version)?,
        }
    }
    Ok(())
}

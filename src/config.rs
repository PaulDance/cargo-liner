use std::collections::BTreeMap;
use std::fs;

use anyhow::Result;
use home::cargo_home;
use semver::VersionReq;
use serde::Deserialize;

use crate::CONFIG_FILE_NAME;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UserConfig {
    pub packages: BTreeMap<String, Package>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Package {
    Simple(VersionReq),
}

pub fn parse_config() -> Result<UserConfig> {
    Ok(toml::from_str(&fs::read_to_string(
        cargo_home()?.join(CONFIG_FILE_NAME),
    )?)?)
}

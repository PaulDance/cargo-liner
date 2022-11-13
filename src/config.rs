//! Module handling all the user configuration deserializing and validating.
//!
//! See [`parse_config`] in order to retrieve such configuration settings from
//! the default file.

use std::collections::BTreeMap;
use std::fs;

use anyhow::Result;
use home::cargo_home;
use semver::VersionReq;
use serde::Deserialize;

use crate::CONFIG_FILE_NAME;

/// Represents the user's configuration deserialized from its file.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UserConfig {
    /// The name-to-setting map for the `packages` section of the config.
    pub packages: BTreeMap<String, Package>,
}

/// Represents the requirement setting configured for a package.
///
/// There is only one variant for now: a version requirement string.
/// The enumeration is deserialized from an untagged form.
#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Package {
    /// Simple form: only a SemVer requirement string.
    Simple(VersionReq),
}

/// Deserializes the user's configuration file and returns the result.
///
/// It may fail on multiple occasions: if Cargo's home may not be found, if the
/// file does not exist, if it cannot be read from or if it is malformed.
pub fn parse_config() -> Result<UserConfig> {
    Ok(toml::from_str(&fs::read_to_string(
        cargo_home()?.join(CONFIG_FILE_NAME),
    )?)?)
}

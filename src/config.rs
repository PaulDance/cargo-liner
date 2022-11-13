use std::collections::BTreeMap;

use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct UserConfig {
    pub packages: BTreeMap<String, Package>,
}

#[derive(Deserialize, Debug)]
#[serde(untagged)]
pub enum Package {
    Simple(String),
}

pub fn parse_config() -> Result<UserConfig> {
    Ok(toml::from_str::<UserConfig>(
        r#"
            [packages]
            a = ""
            b = "*"
            c = "1.2.3"
        "#,
    )?)
}

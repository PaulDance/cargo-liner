//! Module handling all the user configuration deserializing and validating.
//!
//! See [`parse_config`] in order to retrieve such configuration settings from
//! the default file.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use home::cargo_home;
use semver::{Version, VersionReq};
use serde::{Deserialize, Serialize};

/// Represents the user's configuration deserialized from its file.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct UserConfig {
    /// The name-to-setting map for the `packages` section of the config.
    pub packages: BTreeMap<String, Package>,
}

impl UserConfig {
    /// The default name for the configuration file in Cargo's home.
    pub const FILE_NAME: &str = "liner.toml";

    /// Returns the [`PathBuf`] pointing to the associated configuration file.
    pub fn file_path() -> Result<PathBuf> {
        Ok(cargo_home()?.join(Self::FILE_NAME))
    }

    /// Deserializes the user's configuration file and returns the result.
    ///
    /// It may fail on multiple occasions: if Cargo's home may not be found, if
    /// the file does not exist, if it cannot be read from or if it is malformed.
    pub fn parse_file() -> Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(Self::file_path()?)?)?)
    }

    /// Serializes the configuration and saves it to the default file.
    ///
    /// It creates the file if it does not already exist. If it already exists,
    /// contents will be enterily overwritten. Just as [`Self::parse_file`], it
    /// may fail on several occasions.
    pub fn save_file(&self) -> Result<()> {
        fs::write(Self::file_path()?, toml::to_string_pretty(self)?)?;
        Ok(())
    }
}

/// Represents the requirement setting configured for a package.
///
/// There is only one variant for now: a version requirement string.
/// The enumeration is deserialized from an untagged form.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
#[serde(untagged)]
pub enum Package {
    /// Simple form: only a SemVer requirement string.
    Simple(VersionReq),
}

/// Representation of the `$CARGO_HOME/.crates.toml` Cargo-managed save file.
#[derive(Deserialize, Debug, PartialEq, Eq)]
pub struct CargoCratesToml {
    #[serde(rename = "v1")]
    pub package_bins: BTreeMap<CargoCratesPackage, Vec<String>>,
}

impl CargoCratesToml {
    /// The default name for the save file in Cargo's home.
    pub const FILE_NAME: &str = ".crates.toml";

    /// Returns the [`PathBuf`] pointing to the associated save file.
    pub fn file_path() -> Result<PathBuf> {
        Ok(cargo_home()?.join(Self::FILE_NAME))
    }

    /// Parse and return a representation of the `$CARGO_HOME/.crates.toml`
    /// Cargo-managed save file.
    pub fn parse_file() -> Result<Self> {
        Ok(toml::from_str(&fs::read_to_string(Self::file_path()?)?)?)
    }

    /// Converts this toml document into a simple user config containing no
    /// particular version requirement, only stars are used.
    pub fn into_versionless_config(self) -> UserConfig {
        UserConfig {
            packages: self
                .package_bins
                .into_iter()
                .map(|(pkg, _)| (pkg.name, Package::Simple(VersionReq::STAR)))
                .collect(),
        }
    }
}

/// Representation of keys of the `v1` table parsed by [`CargoCratesToml`].
#[derive(Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
#[serde(try_from = "String")]
pub struct CargoCratesPackage {
    pub name: String,
    pub version: Version,
    pub source: String,
}

/// Deserialize by splitting by spaces, isolating the name, parsing the version
/// and trimming the parentheses around the source.
impl TryFrom<String> for CargoCratesPackage {
    type Error = anyhow::Error;

    fn try_from(s: String) -> Result<Self> {
        let mut parts = s.splitn(3, ' ');
        Ok(Self {
            name: parts
                .next()
                .ok_or_else(|| anyhow!("Missing name"))?
                .to_owned(),
            version: parts
                .next()
                .ok_or_else(|| anyhow!("Missing version"))?
                .parse::<Version>()?,
            source: parts
                .next()
                .ok_or_else(|| anyhow!("Missing source"))?
                .trim_start_matches('(')
                .trim_end_matches(')')
                .to_owned(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deser_userconfig_empty_iserr() {
        assert!(toml::from_str::<UserConfig>("").is_err());
    }

    #[test]
    fn test_deser_userconfig_no_packages() {
        assert_eq!(
            toml::from_str::<UserConfig>(
                r#"
                    [packages]
                "#,
            )
            .unwrap(),
            UserConfig {
                packages: BTreeMap::new(),
            },
        );
    }

    #[test]
    fn test_deser_userconfig_simple_versions() {
        assert_eq!(
            toml::from_str::<UserConfig>(
                r#"
                    [packages]
                    a = "1.2.3"
                    b = "1.2"
                    c = "1"
                    d = "*"
                    e = "1.*"
                    f = "1.2.*"
                    g = "~1.2"
                    h = "~1"
                "#,
            )
            .unwrap(),
            UserConfig {
                packages: [
                    ("a", "1.2.3"),
                    ("b", "1.2"),
                    ("c", "1"),
                    ("d", "*"),
                    ("e", "1.*"),
                    ("f", "1.2.*"),
                    ("g", "~1.2"),
                    ("h", "~1")
                ]
                .into_iter()
                .map(|(name, version)| (
                    name.to_owned(),
                    Package::Simple(VersionReq::parse(version).unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
            }
        );
    }

    #[test]
    fn test_deser_cargocrates_empty_iserr() {
        assert!(toml::from_str::<CargoCratesToml>("").is_err());
    }

    #[test]
    fn test_deser_cargocrates_no_packages() {
        assert_eq!(
            toml::from_str::<CargoCratesToml>(
                r#"
                    [v1]
                "#,
            )
            .unwrap(),
            CargoCratesToml {
                package_bins: BTreeMap::new(),
            },
        );
    }

    #[test]
    fn test_deser_cargocrates_full_versions() {
        assert_eq!(
            toml::from_str::<CargoCratesToml>(
                r#"
                    [v1]
                    "a 1.2.3 (registry+https://github.com/rust-lang/crates.io-index)" = ["a"]
                    "b 0.1.2 (registry+https://github.com/rust-lang/crates.io-index)" = ["b1", "b2"]
                    "c 0.0.0 (path+file:///a/b/c)" = ["c1", "c2", "c3"]
                "#,
            )
            .unwrap(),
            CargoCratesToml {
                package_bins: [
                    (
                        "a",
                        "1.2.3",
                        "registry+https://github.com/rust-lang/crates.io-index",
                        vec!["a"],
                    ),
                    (
                        "b",
                        "0.1.2",
                        "registry+https://github.com/rust-lang/crates.io-index",
                        vec!["b1", "b2"],
                    ),
                    ("c", "0.0.0", "path+file:///a/b/c", vec!["c1", "c2", "c3"]),
                ]
                .into_iter()
                .map(|(name, version, source, bins)| (
                    CargoCratesPackage {
                        name: name.to_owned(),
                        version: version.parse::<Version>().unwrap(),
                        source: source.to_owned(),
                    },
                    bins.into_iter().map(str::to_owned).collect::<Vec<_>>(),
                ))
                .collect::<BTreeMap<_, _>>(),
            }
        );
    }
}

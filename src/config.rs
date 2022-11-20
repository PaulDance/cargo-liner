//! Module handling all the user configuration deserializing and validating.
//!
//! See [`UserConfig::parse_file`] in order to retrieve such configuration
//! settings from the default file.

use std::collections::BTreeMap;
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use home::cargo_home;
use semver::{Op, Version, VersionReq};
use serde::{Deserialize, Serialize};

/// Represents the user's configuration deserialized from its file.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
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
        Ok(toml::from_str::<Self>(&fs::read_to_string(Self::file_path()?)?)?.self_update(true))
    }

    /// Serializes the configuration and saves it to the default file.
    ///
    /// It creates the file if it does not already exist. If it already exists,
    /// contents will be enterily overwritten. Just as [`Self::parse_file`], it
    /// may fail on several occasions.
    pub fn save_file(&self) -> Result<()> {
        fs::write(Self::file_path()?, self.to_string_pretty()?)?;
        Ok(())
    }

    /// Converts the config to a pretty TOML string with literal strings disabled.
    fn to_string_pretty(&self) -> Result<String> {
        let mut dst = String::new();
        self.serialize(toml::Serializer::pretty(&mut dst).pretty_string_literal(false))?;
        Ok(dst)
    }

    /// Enable or disable self-updating.
    ///
    /// If `sup` is `true` and the current crate is not already contained in
    /// the configured packages, then it will add it, otherwise remove it.
    pub fn self_update(mut self, sup: bool) -> Self {
        if sup {
            if !self.packages.contains_key(clap::crate_name!()) {
                self.packages
                    .insert(clap::crate_name!().to_owned(), Package::SIMPLE_STAR);
            }
        } else {
            self.packages.remove(clap::crate_name!());
        }
        self
    }
}

/// Represents the requirement setting configured for a package.
///
/// There is only one variant for now: a version requirement string.
/// The enumeration is deserialized from an untagged form.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(untagged)]
pub enum Package {
    /// Simple form: only a SemVer requirement string.
    Simple(VersionReq),
}

impl Package {
    /// Convenience shortcut for simple and star version requirement package.
    pub const SIMPLE_STAR: Self = Self::Simple(VersionReq::STAR);
}

/// Representation of the `$CARGO_HOME/.crates.toml` Cargo-managed save file.
#[derive(Deserialize, Debug, Clone, PartialEq, Eq)]
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

    /// Converts this toml document into a custom user config by mapping
    /// listed packages using the given `pkg_map` function.
    ///
    /// The function must take in by value the couple of [`CargoCratesPackage`]
    /// to vector of binary names and return a couple of package name to
    /// [`Package`] information.
    pub fn into_config<F>(self, pkg_map: F) -> UserConfig
    where
        F: FnMut((CargoCratesPackage, Vec<String>)) -> (String, Package),
    {
        UserConfig {
            packages: self.package_bins.into_iter().map(pkg_map).collect(),
        }
    }

    /// Converts this toml document into a simple user config containing no
    /// particular version requirement, only stars are used.
    pub fn into_star_version_config(self) -> UserConfig {
        self.into_config(|(pkg, _)| (pkg.name, Package::SIMPLE_STAR))
    }

    /// Converts the config by turning versions to requirements using the given
    /// comparison operator.
    fn into_op_version_config(self, op: Op) -> UserConfig {
        self.into_config(|(pkg, _)| (pkg.name, Package::Simple(ver_to_req(&pkg.version, op))))
    }

    /// Converts this toml document into a simple user config containing full
    /// and exact version requirements.
    pub fn into_exact_version_config(self) -> UserConfig {
        self.into_op_version_config(Op::Exact)
    }

    /// Converts this toml document into a simple user config containing
    /// compatible version requirements, i.e. with the caret operator.
    pub fn into_comp_version_config(self) -> UserConfig {
        self.into_op_version_config(Op::Caret)
    }

    /// Converts this toml document into a simple user config containing
    /// patch version requirements, i.e. with the tilde operator.
    pub fn into_patch_version_config(self) -> UserConfig {
        self.into_op_version_config(Op::Tilde)
    }
}

/// Representation of keys of the `v1` table parsed by [`CargoCratesToml`].
#[derive(Deserialize, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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

/// Converts the given version to a version requirement with the given operator.
fn ver_to_req(ver: &Version, op: Op) -> VersionReq {
    let mut req = ver.to_string().parse::<VersionReq>().unwrap();
    req.comparators[0].op = op;
    req
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;
    use std::iter;

    #[test]
    fn test_deser_userconfig_empty_iserr() {
        assert!(toml::from_str::<UserConfig>("").is_err());
    }

    #[test]
    fn test_deser_userconfig_no_packages() {
        assert_eq!(
            toml::from_str::<UserConfig>("[packages]\n").unwrap(),
            UserConfig::default(),
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
    fn test_userconfig_tostringpretty_no_packages() {
        assert_eq!(
            UserConfig::default().to_string_pretty().unwrap(),
            "[packages]\n",
        );
    }

    #[test]
    fn test_userconfig_tostringpretty_simple_versions() {
        assert_eq!(
            UserConfig {
                packages: [
                    ("a", "1.2.3"),
                    ("b", "1.2"),
                    ("c", "1"),
                    ("d", "*"),
                    ("e", "1.*"),
                    ("f", "1.2.*"),
                    ("g", "~1.2"),
                    ("h", "~1"),
                ]
                .into_iter()
                .map(|(name, version)| (
                    name.to_owned(),
                    Package::Simple(VersionReq::parse(version).unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
            }
            .to_string_pretty()
            .unwrap(),
            indoc!(
                r#"
                    [packages]
                    a = "^1.2.3"
                    b = "^1.2"
                    c = "^1"
                    d = "*"
                    e = "1.*"
                    f = "1.2.*"
                    g = "~1.2"
                    h = "~1"
                "#,
            ),
        );
    }

    #[test]
    fn test_userconfig_selfupdate_enable() {
        assert_eq!(
            UserConfig::default().self_update(true).packages,
            iter::once(("cargo-liner".to_owned(), Package::SIMPLE_STAR))
                .collect::<BTreeMap<_, _>>(),
        );
    }

    #[test]
    fn test_userconfig_selfupdate_enable_noreplace() {
        let pkgs = iter::once((
            "cargo-liner".to_owned(),
            Package::Simple(VersionReq::parse("1.2.3").unwrap()),
        ))
        .collect::<BTreeMap<_, _>>();

        assert_eq!(
            UserConfig {
                packages: pkgs.clone(),
            }
            .self_update(true)
            .packages,
            pkgs,
        );
    }

    #[test]
    fn test_userconfig_selfupdate_disable_star() {
        assert_eq!(
            UserConfig::default()
                .self_update(true)
                .self_update(false)
                .packages,
            BTreeMap::new(),
        );
    }

    #[test]
    fn test_userconfig_selfupdate_disable_nostar() {
        assert_eq!(
            UserConfig {
                packages: iter::once((
                    "cargo-liner".to_owned(),
                    Package::Simple(VersionReq::parse("1.2.3").unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
            }
            .self_update(false)
            .packages,
            BTreeMap::new(),
        );
    }

    #[test]
    fn test_deser_cargocrates_empty_iserr() {
        assert!(toml::from_str::<CargoCratesToml>("").is_err());
    }

    #[test]
    fn test_deser_cargocrates_no_packages() {
        assert_eq!(
            toml::from_str::<CargoCratesToml>("[v1]\n").unwrap(),
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
                    "a 1.2.3 (registry+https://example.com/index)" = ["a"]
                    "b 0.1.2 (registry+https://example.com/index)" = ["b1", "b2"]
                    "c 0.0.0 (path+file:///a/b/c)" = ["c1", "c2", "c3"]
                "#,
            )
            .unwrap(),
            CargoCratesToml {
                package_bins: [
                    (
                        "a",
                        "1.2.3",
                        "registry+https://example.com/index",
                        vec!["a"],
                    ),
                    (
                        "b",
                        "0.1.2",
                        "registry+https://example.com/index",
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

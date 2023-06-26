use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;

use anyhow::{anyhow, Result};
use home::cargo_home;
use semver::{Op, Version, VersionReq};
use serde::Deserialize;

use super::{Package, UserConfig};

/// Representation of the `$CARGO_HOME/.crates.toml` Cargo-managed save file.
#[derive(Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct CargoCratesToml {
    #[serde(rename = "v1")]
    pub package_bins: BTreeMap<CargoCratesPackage, Vec<String>>,
}

impl CargoCratesToml {
    /// The default name for the save file in Cargo's home.
    pub const FILE_NAME: &str = ".crates.toml";

    /// Returns the [`PathBuf`] pointing to the associated save file.
    pub fn file_path() -> Result<PathBuf> {
        debug!("Building file path...");
        Ok(cargo_home()?.join(Self::FILE_NAME))
    }

    /// Parse and return a representation of the `$CARGO_HOME/.crates.toml`
    /// Cargo-managed save file.
    pub fn parse_file() -> Result<Self> {
        let path = Self::file_path()?;
        debug!("Reading Cargo-installed packages from {:?}...", &path);
        let info_str = fs::read_to_string(path)?;
        trace!("Read {} bytes.", info_str.len());
        trace!("Got: {:?}.", &info_str);
        debug!("Deserializing packages...");
        let info = toml::from_str(&info_str)?;
        trace!("Got: {:?}.", &info);
        Ok(info)
    }

    /// Consumes the document and returns the set of installed package names.
    pub fn into_names(self) -> BTreeSet<String> {
        self.package_bins.into_keys().map(|pkg| pkg.name).collect()
    }

    /// Converts this toml document into a custom user config by mapping
    /// listed packages using the given `pkg_map` function.
    ///
    /// The function must take in by value the couple of [`CargoCratesPackage`]
    /// to vector of binary names and return a couple of package name to
    /// [`Package`] information.
    ///
    /// The current crate will be kept in the packages iff `keep_self` is
    /// `true`, otherwise it will be filtered out.
    fn into_config<F>(self, pkg_map: F, keep_self: bool) -> UserConfig
    where
        F: FnMut((CargoCratesPackage, Vec<String>)) -> (String, Package),
    {
        UserConfig {
            packages: self
                .package_bins
                .into_iter()
                .filter(|(pkg, _)| keep_self || pkg.name != clap::crate_name!())
                .map(pkg_map)
                .collect(),
        }
    }

    /// Converts this toml document into a simple user config containing no
    /// particular version requirement, only stars are used.
    pub fn into_star_version_config(self, keep_self: bool) -> UserConfig {
        debug!("Converting packages to config with op: \"*\"...");
        self.into_config(|(pkg, _)| (pkg.name, Package::SIMPLE_STAR), keep_self)
    }

    /// Converts the config by turning versions to requirements using the given
    /// comparison operator.
    ///
    /// Filters the current crate out of the resulting configuration's packages.
    fn into_op_version_config(self, op: Op, keep_self: bool) -> UserConfig {
        debug!("Converting packages to config with op: {:?}...", &op);
        self.into_config(
            |(pkg, _)| (pkg.name, Package::Simple(ver_to_req(&pkg.version, op))),
            keep_self,
        )
    }

    /// Converts this toml document into a simple user config containing full
    /// and exact version requirements.
    pub fn into_exact_version_config(self, keep_self: bool) -> UserConfig {
        self.into_op_version_config(Op::Exact, keep_self)
    }

    /// Converts this toml document into a simple user config containing
    /// compatible version requirements, i.e. with the caret operator.
    pub fn into_comp_version_config(self, keep_self: bool) -> UserConfig {
        self.into_op_version_config(Op::Caret, keep_self)
    }

    /// Converts this toml document into a simple user config containing
    /// patch version requirements, i.e. with the tilde operator.
    pub fn into_patch_version_config(self, keep_self: bool) -> UserConfig {
        self.into_op_version_config(Op::Tilde, keep_self)
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
    use std::iter;

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

    fn cargocrates_example1() -> CargoCratesToml {
        toml::from_str::<CargoCratesToml>(
            r#"
                [v1]
                "a 1.2.3 (registry+https://example.com/index)" = ["a"]
                "b 0.1.2 (registry+https://example.com/index)" = ["b1", "b2"]
                "c 0.0.0 (path+file:///a/b/c)" = ["c1", "c2", "c3"]
                "cargo-liner 0.2.1 (registry+https://crates.io/index)" = ["cargo-liner"]
            "#,
        )
        .unwrap()
    }

    #[test]
    fn test_deser_cargocrates_full_versions() {
        assert_eq!(
            cargocrates_example1(),
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
                    (
                        "cargo-liner",
                        "0.2.1",
                        "registry+https://crates.io/index",
                        vec!["cargo-liner"]
                    ),
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

    #[test]
    fn test_cargocrates_intostarcfg_no_packages() {
        assert_eq!(
            CargoCratesToml::default().into_star_version_config(false),
            UserConfig::default(),
        );
    }

    #[test]
    fn test_cargocrates_intostarcfg_noself() {
        assert!(!CargoCratesToml {
            package_bins: iter::once((
                CargoCratesPackage {
                    name: clap::crate_name!().to_owned(),
                    version: "1.2.3".parse().unwrap(),
                    source: String::new()
                },
                vec![],
            ))
            .collect()
        }
        .into_star_version_config(false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intostarcfg_full_versions() {
        assert_eq!(
            cargocrates_example1().into_star_version_config(false),
            UserConfig {
                packages: [("a", "*"), ("b", "*"), ("c", "*")]
                    .into_iter()
                    .map(|(name, version)| (
                        name.to_owned(),
                        Package::Simple(VersionReq::parse(version).unwrap()),
                    ))
                    .collect::<BTreeMap<_, _>>(),
            },
        );
    }

    #[test]
    fn test_cargocrates_intoexactcfg_noself() {
        assert!(!CargoCratesToml {
            package_bins: iter::once((
                CargoCratesPackage {
                    name: clap::crate_name!().to_owned(),
                    version: "1.2.3".parse().unwrap(),
                    source: String::new()
                },
                vec![],
            ))
            .collect()
        }
        .into_exact_version_config(false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intoexactcfg_full_versions() {
        assert_eq!(
            cargocrates_example1().into_exact_version_config(false),
            UserConfig {
                packages: [("a", "=1.2.3"), ("b", "=0.1.2"), ("c", "=0.0.0")]
                    .into_iter()
                    .map(|(name, version)| (
                        name.to_owned(),
                        Package::Simple(VersionReq::parse(version).unwrap()),
                    ))
                    .collect::<BTreeMap<_, _>>(),
            },
        );
    }

    #[test]
    fn test_cargocrates_intocompcfg_noself() {
        assert!(!CargoCratesToml {
            package_bins: iter::once((
                CargoCratesPackage {
                    name: clap::crate_name!().to_owned(),
                    version: "1.2.3".parse().unwrap(),
                    source: String::new()
                },
                vec![],
            ))
            .collect()
        }
        .into_comp_version_config(false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intocompcfg_full_versions() {
        assert_eq!(
            cargocrates_example1().into_comp_version_config(false),
            UserConfig {
                packages: [("a", "^1.2.3"), ("b", "^0.1.2"), ("c", "^0.0.0")]
                    .into_iter()
                    .map(|(name, version)| (
                        name.to_owned(),
                        Package::Simple(VersionReq::parse(version).unwrap()),
                    ))
                    .collect::<BTreeMap<_, _>>(),
            },
        );
    }

    #[test]
    fn test_cargocrates_intopatchcfg_noself() {
        assert!(!CargoCratesToml {
            package_bins: iter::once((
                CargoCratesPackage {
                    name: clap::crate_name!().to_owned(),
                    version: "1.2.3".parse().unwrap(),
                    source: String::new()
                },
                vec![],
            ))
            .collect()
        }
        .into_patch_version_config(false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intopatchcfg_full_versions() {
        assert_eq!(
            cargocrates_example1().into_patch_version_config(false),
            UserConfig {
                packages: [("a", "~1.2.3"), ("b", "~0.1.2"), ("c", "~0.0.0")]
                    .into_iter()
                    .map(|(name, version)| (
                        name.to_owned(),
                        Package::Simple(VersionReq::parse(version).unwrap()),
                    ))
                    .collect::<BTreeMap<_, _>>(),
            },
        );
    }

    #[test]
    fn test_cargocrates_intostarcfg_nopackages_keepself() {
        assert_eq!(
            CargoCratesToml::default().into_star_version_config(true),
            UserConfig::default(),
        );
    }

    #[test]
    fn test_cargocrates_intostarcfg_keepself() {
        assert!(CargoCratesToml {
            package_bins: iter::once((
                CargoCratesPackage {
                    name: clap::crate_name!().to_owned(),
                    version: "1.2.3".parse().unwrap(),
                    source: String::new()
                },
                vec![],
            ))
            .collect()
        }
        .into_star_version_config(true)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intostarcfg_fullversions_keepself() {
        assert_eq!(
            cargocrates_example1().into_star_version_config(true),
            UserConfig {
                packages: [
                    ("a", "*"),
                    ("b", "*"),
                    ("c", "*"),
                    (clap::crate_name!(), "*")
                ]
                .into_iter()
                .map(|(name, version)| (
                    name.to_owned(),
                    Package::Simple(VersionReq::parse(version).unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
            },
        );
    }

    #[test]
    fn test_cargocrates_intoexactcfg_keepself() {
        assert!(CargoCratesToml {
            package_bins: iter::once((
                CargoCratesPackage {
                    name: clap::crate_name!().to_owned(),
                    version: "1.2.3".parse().unwrap(),
                    source: String::new()
                },
                vec![],
            ))
            .collect()
        }
        .into_exact_version_config(true)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intoexactcfg_fullversions_keepself() {
        assert_eq!(
            cargocrates_example1().into_exact_version_config(true),
            UserConfig {
                packages: [
                    ("a", "=1.2.3"),
                    ("b", "=0.1.2"),
                    ("c", "=0.0.0"),
                    (clap::crate_name!(), "=0.2.1")
                ]
                .into_iter()
                .map(|(name, version)| (
                    name.to_owned(),
                    Package::Simple(VersionReq::parse(version).unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
            },
        );
    }

    #[test]
    fn test_cargocrates_intocompcfg_keepself() {
        assert!(CargoCratesToml {
            package_bins: iter::once((
                CargoCratesPackage {
                    name: clap::crate_name!().to_owned(),
                    version: "1.2.3".parse().unwrap(),
                    source: String::new()
                },
                vec![],
            ))
            .collect()
        }
        .into_comp_version_config(true)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intocompcfg_fullversions_keepself() {
        assert_eq!(
            cargocrates_example1().into_comp_version_config(true),
            UserConfig {
                packages: [
                    ("a", "^1.2.3"),
                    ("b", "^0.1.2"),
                    ("c", "^0.0.0"),
                    (clap::crate_name!(), "^0.2.1")
                ]
                .into_iter()
                .map(|(name, version)| (
                    name.to_owned(),
                    Package::Simple(VersionReq::parse(version).unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
            },
        );
    }

    #[test]
    fn test_cargocrates_intopatchcfg_keepself() {
        assert!(CargoCratesToml {
            package_bins: iter::once((
                CargoCratesPackage {
                    name: clap::crate_name!().to_owned(),
                    version: "1.2.3".parse().unwrap(),
                    source: String::new()
                },
                vec![],
            ))
            .collect()
        }
        .into_patch_version_config(true)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intopatchcfg_fullversions_keepself() {
        assert_eq!(
            cargocrates_example1().into_patch_version_config(true),
            UserConfig {
                packages: [
                    ("a", "~1.2.3"),
                    ("b", "~0.1.2"),
                    ("c", "~0.0.0"),
                    (clap::crate_name!(), "~0.2.1")
                ]
                .into_iter()
                .map(|(name, version)| (
                    name.to_owned(),
                    Package::Simple(VersionReq::parse(version).unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
            },
        );
    }
}

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;

use color_eyre::eyre::{self, eyre, Result, WrapErr};
use color_eyre::Section;
use semver::{Op, Version, VersionReq};
use serde::Deserialize;
use serde_with::DeserializeFromStr;
use url::Url;

use super::{Package, UserConfig};
use crate::cargo;

/// Representation of the `$CARGO_HOME/.crates.toml` Cargo-managed save file.
#[derive(Debug, Default, Clone, PartialEq, Eq, Deserialize)]
pub struct CargoCratesToml {
    #[serde(rename = "v1")]
    pub package_bins: BTreeMap<CargoCratesPackage, Vec<String>>,
}

impl CargoCratesToml {
    /// The default name for the save file in Cargo's home.
    pub const FILE_NAME: &'static str = ".crates.toml";

    /// Returns the [`PathBuf`] pointing to the associated save file.
    ///
    /// In order to determine which exact file to get, it will first try to use
    /// `$CARGO_INSTALL_ROOT` if available, or then fall back to `$CARGO_HOME`
    /// otherwise.
    pub fn file_path() -> Result<PathBuf> {
        const INSTALL_ROOT_CONFIG_KEY: &str = "install.root";
        log::debug!("Building file path...");

        // Don't particularly filter: default to `$CARGO_HOME` on any error.
        match cargo::config_get(INSTALL_ROOT_CONFIG_KEY) {
            Ok(install_root_path) => Ok(install_root_path
                .parse::<PathBuf>()
                .wrap_err("Failed to parse the install.root Cargo config as a path.")
                .suggestion("Check in Cargo's config if the value is a well-formed path.")?
                .join(Self::FILE_NAME)),
            Err(err) => {
                log::debug!(
                    "Failed to retrieve `{}` from Cargo's configuration on error: {err:#?}.",
                    INSTALL_ROOT_CONFIG_KEY,
                );
                log::debug!("Defaulting to Cargo's home directory...");
                Ok(cargo::home()?.join(Self::FILE_NAME))
            }
        }
    }

    /// Parse and return a representation of the `$CARGO_HOME/.crates.toml`
    /// Cargo-managed save file.
    pub fn parse_file() -> Result<Self> {
        let path = Self::file_path().wrap_err("Failed to build Cargo's .crates.toml file path.")?;
        log::debug!("Reading Cargo-installed packages from {path:#?}...");
        let info_str = fs::read_to_string(path)
            .wrap_err("Failed to read Cargo's .crates.toml file.")
            .note("This can happen for many reasons.")
            .suggestion("Check if the file exists and has the correct permissions.")?;
        log::trace!("Read {} bytes.", info_str.len());
        log::trace!("Got: {info_str:#?}.");
        log::debug!("Deserializing packages...");
        let info = toml::from_str(&info_str)
            .wrap_err("Failed to deserialize Cargo's .crates.toml file contents.")
            .note("This should not easily happen as the file is automatically maintained by Cargo.")
            .suggestion("Check if it is corrupted in some way.")?;
        log::trace!("Got: {info:#?}.");
        Ok(info)
    }

    /// Consumes the document and returns the set of installed package names.
    pub fn into_names(self) -> BTreeSet<String> {
        self.package_bins.into_keys().map(|pkg| pkg.name).collect()
    }

    /// Consumes the document and returns the set of installed package names
    /// to versions.
    pub fn into_name_versions(self) -> BTreeMap<String, Version> {
        self.package_bins
            .into_keys()
            .map(|pkg| (pkg.name, pkg.version))
            .collect()
    }

    /// Converts this toml document into a custom user config by mapping
    /// listed packages using the given `pkg_map` function.
    ///
    /// The function must take in by value the couple of [`CargoCratesPackage`]
    /// to vector of binary names and return a couple of package name to
    /// [`Package`] information.
    ///
    /// The current crate will be kept in the packages if `keep_self` is
    /// `true`, otherwise it will be filtered out.
    ///
    /// All locally-installed packages are kept if `keep_local` is
    /// `true`, otherwise they will be filtered out.
    fn into_config<F>(self, pkg_map: F, keep_self: bool, keep_local: bool) -> UserConfig
    where
        F: FnMut((CargoCratesPackage, Vec<String>)) -> (String, Package),
    {
        UserConfig {
            packages: self
                .package_bins
                .into_iter()
                .filter(|(pkg, _)| {
                    (keep_local || pkg.source.kind != SourceKind::Path)
                        && (keep_self || pkg.name != clap::crate_name!())
                })
                .map(pkg_map)
                .collect(),
        }
    }

    /// Converts this toml document into a simple user config containing no
    /// particular version requirement, only stars are used.
    pub fn into_star_version_config(self, keep_self: bool, keep_local: bool) -> UserConfig {
        log::debug!("Converting packages to config with op: \"*\"...");
        self.into_config(
            |(pkg, _)| (pkg.name, Package::SIMPLE_STAR),
            keep_self,
            keep_local,
        )
    }

    /// Converts the config by turning versions to requirements using the given
    /// comparison operator.
    ///
    /// Filters the current crate out of the resulting configuration's packages.
    fn into_op_version_config(self, op: Op, keep_self: bool, keep_local: bool) -> UserConfig {
        log::debug!("Converting packages to config with op: {op:#?}...");
        self.into_config(
            |(pkg, _)| (pkg.name, Package::Simple(ver_to_req(&pkg.version, op))),
            keep_self,
            keep_local,
        )
    }

    /// Converts this toml document into a simple user config containing full
    /// and exact version requirements.
    pub fn into_exact_version_config(self, keep_self: bool, keep_local: bool) -> UserConfig {
        self.into_op_version_config(Op::Exact, keep_self, keep_local)
    }

    /// Converts this toml document into a simple user config containing
    /// compatible version requirements, i.e. with the caret operator.
    pub fn into_comp_version_config(self, keep_self: bool, keep_local: bool) -> UserConfig {
        self.into_op_version_config(Op::Caret, keep_self, keep_local)
    }

    /// Converts this toml document into a simple user config containing
    /// patch version requirements, i.e. with the tilde operator.
    pub fn into_patch_version_config(self, keep_self: bool, keep_local: bool) -> UserConfig {
        self.into_op_version_config(Op::Tilde, keep_self, keep_local)
    }
}

/// Representation of keys of the `v1` table parsed by [`CargoCratesToml`].
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr)]
pub struct CargoCratesPackage {
    pub name: String,
    pub version: Version,
    pub source: PackageSource,
}

/// Deserialize by splitting by spaces, isolating the name, parsing the version
/// and trimming the parentheses around the source.
impl FromStr for CargoCratesPackage {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(3, ' ');
        let name = parts
            .next()
            .ok_or_else(|| eyre!("Missing name."))?
            .to_owned();

        Ok(Self {
            version: parts
                .next()
                .ok_or_else(|| eyre!("Missing version for {name:?}."))?
                .parse()
                .wrap_err_with(|| format!("Failed to parse the version for {name:?}."))?,
            source: parts
                .next()
                .ok_or_else(|| eyre!("Missing source for {name:?}."))?
                .trim_start_matches('(')
                .trim_end_matches(')')
                .parse()
                .wrap_err_with(|| format!("Failed to parse the source for {name:?}."))?,
            name,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr)]
pub struct PackageSource {
    pub kind: SourceKind,
    pub url: Url,
}

impl FromStr for PackageSource {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        let mut parts = s.splitn(2, '+');
        Ok(Self {
            kind: parts
                .next()
                .ok_or_else(|| eyre!("Missing source origin."))?
                .parse()
                .wrap_err("Failed to parse the source kind.")?,
            url: parts
                .next()
                .ok_or_else(|| eyre!("Missing source path."))?
                .parse()
                .wrap_err("Failed to parse the source URL.")?,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, DeserializeFromStr)]
pub enum SourceKind {
    Git,
    Path,
    Registry,
    SparseRegistry,
}

impl FromStr for SourceKind {
    type Err = eyre::Error;

    fn from_str(s: &str) -> Result<Self> {
        Ok(match s {
            "git" => Self::Git,
            "path" => Self::Path,
            "registry" => Self::Registry,
            "sparse" => Self::SparseRegistry,
            kind => eyre::bail!("Unsupported source protocol: {}", kind),
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
    use std::iter;

    use super::*;

    impl PackageSource {
        fn crates_io() -> Self {
            Self {
                kind: SourceKind::Registry,
                url: "https://github.com/rust-lang/crates.io-index"
                    .parse()
                    .unwrap(),
            }
        }
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
                        "registry",
                        "https://example.com/index",
                        vec!["a"],
                    ),
                    (
                        "b",
                        "0.1.2",
                        "registry",
                        "https://example.com/index",
                        vec!["b1", "b2"],
                    ),
                    (
                        "c",
                        "0.0.0",
                        "path",
                        "file:///a/b/c",
                        vec!["c1", "c2", "c3"],
                    ),
                    (
                        "cargo-liner",
                        "0.2.1",
                        "registry",
                        "https://crates.io/index",
                        vec!["cargo-liner"],
                    ),
                ]
                .into_iter()
                .map(|(name, version, source_kind, source_url, bins)| (
                    CargoCratesPackage {
                        name: name.to_owned(),
                        version: version.parse::<Version>().unwrap(),
                        source: PackageSource {
                            kind: source_kind.parse().unwrap(),
                            url: source_url.parse().unwrap(),
                        },
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
            CargoCratesToml::default().into_star_version_config(false, false),
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
                    source: PackageSource::crates_io(),
                },
                vec![],
            ))
            .collect()
        }
        .into_star_version_config(false, false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intostarcfg_full_versions() {
        assert_eq!(
            cargocrates_example1().into_star_version_config(false, false),
            UserConfig {
                packages: [("a", "*"), ("b", "*")]
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
                    source: PackageSource::crates_io(),
                },
                vec![],
            ))
            .collect()
        }
        .into_exact_version_config(false, false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intoexactcfg_full_versions() {
        assert_eq!(
            cargocrates_example1().into_exact_version_config(false, false),
            UserConfig {
                packages: [("a", "=1.2.3"), ("b", "=0.1.2")]
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
                    source: PackageSource::crates_io(),
                },
                vec![],
            ))
            .collect()
        }
        .into_comp_version_config(false, false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intocompcfg_full_versions() {
        assert_eq!(
            cargocrates_example1().into_comp_version_config(false, false),
            UserConfig {
                packages: [("a", "^1.2.3"), ("b", "^0.1.2")]
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
                    source: PackageSource::crates_io(),
                },
                vec![],
            ))
            .collect()
        }
        .into_patch_version_config(false, false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intopatchcfg_full_versions() {
        assert_eq!(
            cargocrates_example1().into_patch_version_config(false, false),
            UserConfig {
                packages: [("a", "~1.2.3"), ("b", "~0.1.2")]
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
            CargoCratesToml::default().into_star_version_config(true, false),
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
                    source: PackageSource::crates_io(),
                },
                vec![],
            ))
            .collect()
        }
        .into_star_version_config(true, false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intostarcfg_fullversions_keeplocal() {
        assert_eq!(
            cargocrates_example1().into_star_version_config(false, true),
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
    fn test_cargocrates_intostarcfg_fullversions_keepself() {
        assert_eq!(
            cargocrates_example1().into_star_version_config(true, false),
            UserConfig {
                packages: [("a", "*"), ("b", "*"), (clap::crate_name!(), "*")]
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
    fn test_cargocrates_intostarcfg_fullversions_keepself_keeplocal() {
        assert_eq!(
            cargocrates_example1().into_star_version_config(true, true),
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
                    source: PackageSource::crates_io(),
                },
                vec![],
            ))
            .collect()
        }
        .into_exact_version_config(true, false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intoexactcfg_fullversions_keepself() {
        assert_eq!(
            cargocrates_example1().into_exact_version_config(true, false),
            UserConfig {
                packages: [
                    ("a", "=1.2.3"),
                    ("b", "=0.1.2"),
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
                    source: PackageSource::crates_io(),
                },
                vec![],
            ))
            .collect()
        }
        .into_comp_version_config(true, false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intocompcfg_fullversions_keepself() {
        assert_eq!(
            cargocrates_example1().into_comp_version_config(true, false),
            UserConfig {
                packages: [
                    ("a", "^1.2.3"),
                    ("b", "^0.1.2"),
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
                    source: PackageSource::crates_io(),
                },
                vec![],
            ))
            .collect()
        }
        .into_patch_version_config(true, false)
        .packages
        .contains_key(clap::crate_name!()));
    }

    #[test]
    fn test_cargocrates_intopatchcfg_fullversions_keepself() {
        assert_eq!(
            cargocrates_example1().into_patch_version_config(true, false),
            UserConfig {
                packages: [
                    ("a", "~1.2.3"),
                    ("b", "~0.1.2"),
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

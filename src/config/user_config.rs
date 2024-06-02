use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::iter;
use std::path::PathBuf;

use color_eyre::eyre::{Result, WrapErr};
use color_eyre::Section;
use serde::{Deserialize, Serialize};

use super::Package;
use crate::cargo;

/// Represents the user's configuration deserialized from its file.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct UserConfig {
    /// The name-to-setting map for the `packages` section of the config.
    pub packages: BTreeMap<String, Package>,
}

impl UserConfig {
    /// The default name for the configuration file in Cargo's home.
    pub const FILE_NAME: &'static str = "liner.toml";

    /// Returns the [`PathBuf`] pointing to the associated configuration file.
    pub fn file_path() -> Result<PathBuf> {
        log::debug!("Building file path...");
        Ok(cargo::home()?.join(Self::FILE_NAME))
    }

    /// Deserializes the user's configuration file and returns the result.
    ///
    /// It may fail on multiple occasions: if Cargo's home may not be found, if
    /// the file does not exist, if it cannot be read from or if it is
    /// malformed.
    pub fn parse_file() -> Result<Self> {
        let path = Self::file_path().wrap_err("Failed to build the configuration file path.")?;
        log::debug!("Reading configuration from {path:#?}...");
        let config_str = fs::read_to_string(path)
            .wrap_err("Failed to read the configuration file.")
            .note("This can happen for many reasons.")
            .suggestion("Check if the file exists and has the correct permissions.")
            .suggestion(
                "If the file does indeed not exist, it can be automatically created using `import`.",
            )?;
        log::trace!("Read {} bytes.", config_str.len());
        log::trace!("Got: {config_str:#?}.");
        log::debug!("Deserializing contents...");
        let config = toml::from_str::<Self>(&config_str)
            .wrap_err("Failed to deserialize the configuration file contents.")
            .note("This can easily happen as the file is edited manually.")
            .suggestion("Check the file for any typos and syntax errors.")?;
        log::trace!("Got: {config:#?}.");
        Ok(config.self_update(true))
    }

    /// Serializes the configuration and saves it to the default file.
    ///
    /// It creates the file if it does not already exist. If it already exists,
    /// contents will be enterily overwritten. Just as [`Self::parse_file`], it
    /// may fail on several occasions.
    pub fn overwrite_file(&self) -> Result<()> {
        let path = Self::file_path().wrap_err("Failed to build the configuration file path.")?;
        let config_str = self.to_string_pretty()?;
        log::debug!("Overwriting configuration to {path:#?}...");
        fs::write(path, config_str)
            .wrap_err("Failed to write the new configuration file contents.")
            .note("This can happen for many reasons.")
            .suggestion("Check the permissions of Cargo's directory and of the file.")?;
        Ok(())
    }

    /// Serializes the configuration and saves it to the default file without
    /// overwriting it.
    ///
    /// It creates the file if it does not already exist. If it already exists,
    /// it will fail on an appropriate error. Just as [`Self::overwrite_file`],
    /// it may fail on several occasions.
    pub fn save_file(&self) -> Result<()> {
        let path = Self::file_path().wrap_err("Failed to build the configuration file path.")?;
        let config_str = self.to_string_pretty()?;
        log::debug!("Writing configuration to {path:#?}...");
        File::options()
            .read(true)
            .write(true)
            .create_new(true)
            .open(&path)
            .wrap_err_with(|| format!("Failed to open the file at path: {path:?}."))
            .note("This can happen for many reasons.")
            .suggestion(
                "Check Cargo's home permissions and if the file already exists while it should not.",
            )?
            .write_all(config_str.as_bytes())
            .wrap_err_with(|| format!("Failed to write to the opened file at {path:?}."))
            .note("This can happen for many reasons, but it should not happen easily at this point.")
            .suggestion("Read the underlying error message.")?;
        Ok(())
    }

    /// Converts the config to a pretty TOML string with literal strings
    /// disabled.
    fn to_string_pretty(&self) -> Result<String> {
        log::debug!("Serializing configuration...");
        let res = toml::to_string_pretty(self)
            .wrap_err("Failed to serialize the already-parsed user configuration.")
            .note("This should not easily happen.")
            .suggestion(crate::OPEN_ISSUE_MSG)?;
        log::trace!("Got: {res:#?}.");
        Ok(res)
    }

    /// Enable or disable self-updating.
    ///
    /// If `sup` is `true` and the current crate is not already contained in
    /// the configured packages, then it will add it, otherwise remove it.
    pub fn self_update(mut self, sup: bool) -> Self {
        if sup {
            self.packages
                .entry(clap::crate_name!().to_owned())
                .or_insert(Package::SIMPLE_STAR);
            log::debug!("Self-updating enabled.");
        } else {
            self.packages.remove(clap::crate_name!());
            log::debug!("Self-updating disabled.");
        }
        self
    }

    /// Enable or disable updating other packages.
    ///
    /// If `upo` is `false`, then the list of packages is reset to only contain
    /// the current crate: irreversible. Otherwise, nothing is done.
    pub fn update_others(mut self, upo: bool) -> Self {
        if !upo {
            self.packages = iter::once((
                clap::crate_name!().to_owned(),
                self.packages
                    .get(clap::crate_name!())
                    .unwrap_or(&Package::SIMPLE_STAR)
                    .clone(),
            ))
            .collect();
            log::debug!("Updating of other packages disabled.");
        }
        self
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::too_many_lines)]
    use std::iter;

    use indoc::indoc;
    use semver::VersionReq;

    use super::*;

    #[test]
    fn test_deser_userconfig_empty_iserr() {
        assert!(toml::from_str::<UserConfig>("").is_err());
    }

    #[test]
    fn test_deser_userconfig_unknownfields_isok() {
        assert!(toml::from_str::<UserConfig>(
            r#"
                [packages]
                abc = "1.2.3"
                [unknown-section]
                unknown-field = "unknown-value"
            "#
        )
        .is_ok());
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
    fn test_deser_userconfig_detailed_requirements() {
        /// Circumvent tuple restriction.
        #[allow(clippy::struct_excessive_bools)]
        #[derive(Debug, PartialEq, Eq)]
        struct PkgInfo {
            version: String,
            features: Vec<String>,
            all_features: bool,
            default_features: bool,
            index: Option<String>,
            registry: Option<String>,
            git: Option<String>,
            branch: Option<String>,
            tag: Option<String>,
            rev: Option<String>,
            path: Option<String>,
            bins: Vec<String>,
            all_bins: bool,
            examples: Vec<String>,
            all_examples: bool,
            force: bool,
            ignore_rust_version: bool,
            frozen: bool,
            locked: bool,
            extra_arguments: Vec<String>,
            environment: BTreeMap<String, String>,
        }

        let mut packages = toml::from_str::<UserConfig>(
            r#"
                [packages]
                a = "1.2.3"
                b = { version = "1.2", features = [ "foo" ] }
                c = { version = "1.2", features = [ "foo" ], default-features = false }
                d = { version = "1.2", all-features = true }
                e = { version = "1.2" }
                f = { version = "1.2", extra-arguments = [] }
                g = { version = "1.2", extra-arguments = [ "--abc", "--def" ] }
                h = { version = "1.2", environment = {} }
                i = { version = "1.2", environment = { ABC = "def", XYZ = "123" } }
                j = { version = "1.2", index = "http://abc123.com" }
                k = { version = "1.2", registry = "abc123" }
                l = { version = "1.2", git = "https://mygit.com/abc/123" }
                m = { version = "1.2", branch = "master-and-servant" }
                n = { version = "1.2", tag = "tig-tag" }
                o = { version = "1.2", rev = "fakesha1" }
                p = { version = "1.2", path = "/a/b/c" }
                q = { version = "1.2", bins = ["bin1", "bin2"] }
                r = { version = "1.2", all-bins = true }
                s = { version = "1.2", examples = ["ex1", "ex2"] }
                t = { version = "1.2", all-examples = true }
                u = { version = "1.2", force = true }
                v = { version = "1.2", ignore-rust-version = true }
                w = { version = "1.2", frozen = true }
                x = { version = "1.2", locked = true }
            "#,
        )
        .unwrap()
        .packages
        .into_iter()
        .map(|(name, req)| {
            (
                name,
                PkgInfo {
                    version: req.version().to_string(),
                    features: req.features().to_owned(),
                    all_features: req.all_features(),
                    default_features: req.default_features(),
                    index: req.index().map(ToOwned::to_owned),
                    registry: req.registry().map(ToOwned::to_owned),
                    git: req.git().map(ToOwned::to_owned),
                    branch: req.branch().map(ToOwned::to_owned),
                    tag: req.tag().map(ToOwned::to_owned),
                    rev: req.rev().map(ToOwned::to_owned),
                    path: req.path().map(ToOwned::to_owned),
                    bins: req.bins().to_vec(),
                    all_bins: req.all_bins(),
                    examples: req.examples().to_vec(),
                    all_examples: req.all_examples(),
                    force: req.force(),
                    ignore_rust_version: req.ignore_rust_version(),
                    frozen: req.frozen(),
                    locked: req.locked(),
                    extra_arguments: req.extra_arguments().to_owned(),
                    environment: req.environment(),
                },
            )
        })
        .collect::<Vec<_>>();
        packages.sort_by_key(|(k, _)| k.clone());

        let packages: Vec<_> = packages.into_iter().map(|(_, v)| v).collect();

        let expected = [
            PkgInfo {
                version: "^1.2.3".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec!["foo".to_owned()],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec!["foo".to_owned()],
                all_features: false,
                default_features: false,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: true,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec!["--abc".to_owned(), "--def".to_owned()],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: [("ABC", "def"), ("XYZ", "123")]
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: Some("http://abc123.com".to_owned()),
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: Some("abc123".to_owned()),
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: Some("https://mygit.com/abc/123".to_owned()),
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: Some("master-and-servant".to_owned()),
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: Some("tig-tag".to_owned()),
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: Some("fakesha1".to_owned()),
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: Some("/a/b/c".to_owned()),
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec!["bin1".to_owned(), "bin2".to_owned()],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: true,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec!["ex1".to_owned(), "ex2".to_owned()],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: true,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: true,
                ignore_rust_version: false,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: true,
                frozen: false,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: true,
                locked: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
            PkgInfo {
                version: "^1.2".to_owned(),
                features: vec![],
                all_features: false,
                default_features: true,
                index: None,
                registry: None,
                git: None,
                branch: None,
                tag: None,
                rev: None,
                path: None,
                bins: vec![],
                all_bins: false,
                examples: vec![],
                all_examples: false,
                force: false,
                ignore_rust_version: false,
                frozen: false,
                locked: true,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
            },
        ]
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(packages, expected);
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
    fn test_userconfig_onlyselfupdate_enable_isdelete() {
        assert_eq!(
            toml::from_str::<UserConfig>("[packages]\na = \"1.2.3\"")
                .unwrap()
                .self_update(true)
                .update_others(false)
                .packages,
            iter::once(("cargo-liner".to_owned(), Package::SIMPLE_STAR))
                .collect::<BTreeMap<_, _>>(),
        );
    }

    #[test]
    fn test_userconfig_onlyselfupdate_disable_isnop() {
        assert_eq!(
            toml::from_str::<UserConfig>("[packages]\na = \"1.2.3\"")
                .unwrap()
                .self_update(true)
                .update_others(true)
                .packages,
            [
                ("cargo-liner".to_owned(), Package::SIMPLE_STAR),
                (
                    "a".to_owned(),
                    Package::Simple(VersionReq::parse("1.2.3").unwrap())
                )
            ]
            .into_iter()
            .collect::<BTreeMap<_, _>>(),
        );
    }

    #[test]
    fn test_userconfig_onlyselfupdate_enable_isnoreplace() {
        let pkgs = iter::once((
            "cargo-liner".to_owned(),
            Package::Simple(VersionReq::parse("1.2.3").unwrap()),
        ))
        .collect::<BTreeMap<_, _>>();

        assert_eq!(
            UserConfig {
                packages: pkgs.clone(),
            }
            .update_others(false)
            .packages,
            pkgs,
        );
    }
}

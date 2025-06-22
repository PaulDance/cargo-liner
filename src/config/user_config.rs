use std::collections::BTreeMap;
use std::fs::{self, File};
use std::io::Write;
use std::iter;
use std::path::PathBuf;

use color_eyre::Section;
use color_eyre::eyre::{Result, WrapErr};
use serde::{Deserialize, Serialize};

use super::PackageRequirement;
use crate::cargo;
use crate::cli::{JettisonArgs, ShipArgs};

/// Represents the user's configuration deserialized from its file.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
pub struct UserConfig {
    /// The name-to-setting map for the `packages` section of the config.
    pub packages: BTreeMap<String, PackageRequirement>,
    /// The option defaults section.
    #[serde(default)]
    // Use an option to have it be removed during serialization when `None`.
    pub defaults: Option<DefaultsSection>,
}

/// Represents the section of the configuration dedicated to setting CLI option
/// defaults.
#[derive(Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct DefaultsSection {
    /// The sub-section supporting the `ship` command options.
    #[serde(rename = "ship")]
    pub ship_cmd: ShipArgs,
    /// The sub-section supporting the `jettison` command options.
    #[serde(rename = "jettison")]
    pub jettison_cmd: JettisonArgs,
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
                .or_insert(PackageRequirement::SIMPLE_STAR);
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
                    .unwrap_or(&PackageRequirement::SIMPLE_STAR)
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
    #![expect(
        clippy::too_many_lines,
        reason = "Some tests here explicitly detail all tested cases, which can be long."
    )]
    use std::iter;

    use indoc::indoc;
    use semver::VersionReq;

    use super::*;
    use crate::cli::BinstallChoice;
    use crate::config::DetailedPackageReq;

    #[test]
    fn test_deser_userconfig_empty_iserr() {
        assert!(toml::from_str::<UserConfig>("").is_err());
    }

    #[test]
    fn test_deser_userconfig_unknownfields_isok() {
        assert!(
            toml::from_str::<UserConfig>(
                r#"
                    [packages]
                    abc = "1.2.3"
                    [unknown-section]
                    unknown-field = "unknown-value"
                "#
            )
            .is_ok()
        );
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
                    PackageRequirement::Simple(VersionReq::parse(version).unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
                defaults: None,
            }
        );
    }

    #[test]
    fn test_deser_userconfig_detailed_requirements() {
        let mut packages = toml::from_str::<UserConfig>(
            r#"
                [packages]
                a1 = "1.2.3"
                a2 = { version = "1.2", features = [ "foo" ] }
                a3 = { version = "1.2", features = [ "foo" ], default-features = false }
                a4 = { version = "1.2", all-features = true }
                a5 = { version = "1.2" }
                a6 = { version = "1.2", extra-arguments = [] }
                a7 = { version = "1.2", extra-arguments = [ "--abc", "--def" ] }
                a8 = { version = "1.2", environment = {} }
                a9 = { version = "1.2", environment = { ABC = "def", XYZ = "123" } }
                b1 = { version = "1.2", index = "http://abc123.com" }
                b2 = { version = "1.2", registry = "abc123" }
                b3 = { version = "1.2", git = "https://mygit.com/abc/123" }
                b4 = { version = "1.2", branch = "master-and-servant" }
                b5 = { version = "1.2", tag = "tig-tag" }
                b6 = { version = "1.2", rev = "fakesha1" }
                b7 = { version = "1.2", path = "/a/b/c" }
                b8 = { version = "1.2", bins = ["bin1", "bin2"] }
                b9 = { version = "1.2", all-bins = true }
                c1 = { version = "1.2", examples = ["ex1", "ex2"] }
                c2 = { version = "1.2", all-examples = true }
                c3 = { version = "1.2", force = true }
                c4 = { version = "1.2", ignore-rust-version = true }
                c5 = { version = "1.2", frozen = true }
                c6 = { version = "1.2", locked = true }
                c7 = { version = "1.2", offline = true }
                c8 = { version = "1.2", skip-check = true }
                c9 = { version = "1.2", no-fail-fast = true }
                d1 = { version = "1.2", binstall = "auto" }
                d2 = { version = "1.2", binstall = "always" }
                d3 = { version = "1.2", binstall = "never" }
            "#,
        )
        .unwrap()
        .packages
        .into_iter()
        .map(|(name, req)| (name, req.into()))
        .collect::<Vec<(_, DetailedPackageReq)>>();
        packages.sort_by_key(|(k, _)| k.clone());
        let packages: Vec<_> = packages.into_iter().map(|(_, v)| v).collect();

        let expected = [
            DetailedPackageReq {
                version: "^1.2.3".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec!["--abc".to_owned(), "--def".to_owned()],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: [("ABC", "def"), ("XYZ", "123")]
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: true,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: true,
                no_fail_fast: false,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: true,
                binstall: None,
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: Some(BinstallChoice::Auto),
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: Some(BinstallChoice::Always),
            },
            DetailedPackageReq {
                version: "^1.2".parse().unwrap(),
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
                offline: false,
                extra_arguments: vec![],
                environment: BTreeMap::new(),
                skip_check: false,
                no_fail_fast: false,
                binstall: Some(BinstallChoice::Never),
            },
        ]
        .into_iter()
        .collect::<Vec<_>>();

        assert_eq!(packages, expected);
    }

    #[test]
    fn test_userconfig_defaults_none() {
        assert!(
            toml::from_str::<UserConfig>(
                r#"
                    [packages]
                    abc = "*"
                "#
            )
            .unwrap()
            .defaults
            .is_none()
        );
    }

    #[test]
    fn test_userconfig_defaults_empty() {
        assert!(
            toml::from_str::<UserConfig>(
                r#"
                    [packages]
                    abc = "*"
                    [defaults]
                "#
            )
            .unwrap()
            .defaults
            .is_some()
        );
    }

    #[test]
    fn test_userconfig_defaults_empty_equals_semiempty() {
        assert_eq!(
            toml::from_str::<UserConfig>(
                r#"
                    [packages]
                    abc = "*"
                    [defaults]
                "#
            ),
            toml::from_str::<UserConfig>(
                r#"
                    [packages]
                    abc = "*"
                    [defaults.ship]
                "#
            )
        );
    }

    #[test]
    fn test_userconfig_defaults_ship() {
        assert_eq!(
            toml::from_str::<UserConfig>(
                r#"
                    [packages]
                    abc = "*"
                    [defaults.ship]
                    no-self = true
                    no-fail-fast = true
                    skip-check = true
                    force = false
                    dry-run = true
                "#
            )
            .unwrap()
            .defaults
            .unwrap()
            .ship_cmd,
            ShipArgs {
                only_self: None,
                no_self: Some(true),
                no_fail_fast: Some(true),
                skip_check: Some(true),
                force: Some(false),
                dry_run: Some(true),
                binstall: None,
            }
        );
    }

    #[test]
    fn test_userconfig_defaults_ship_binstall() {
        for (val_str, val) in [
            ("auto", BinstallChoice::Auto),
            ("always", BinstallChoice::Always),
            ("never", BinstallChoice::Never),
        ] {
            assert_eq!(
                toml::from_str::<UserConfig>(&format!(
                    r#"
                        [packages]
                        abc = "*"
                        [defaults.ship]
                        binstall = "{val_str}"
                    "#
                ))
                .unwrap()
                .defaults
                .unwrap()
                .ship_cmd,
                ShipArgs {
                    binstall: Some(val),
                    ..Default::default()
                }
            );
        }
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
                    PackageRequirement::Simple(VersionReq::parse(version).unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
                defaults: None,
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
            iter::once(("cargo-liner".to_owned(), PackageRequirement::SIMPLE_STAR))
                .collect::<BTreeMap<_, _>>(),
        );
    }

    #[test]
    fn test_userconfig_selfupdate_enable_noreplace() {
        let pkgs = iter::once((
            "cargo-liner".to_owned(),
            PackageRequirement::Simple(VersionReq::parse("1.2.3").unwrap()),
        ))
        .collect::<BTreeMap<_, _>>();

        assert_eq!(
            UserConfig {
                packages: pkgs.clone(),
                defaults: None,
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
                    PackageRequirement::Simple(VersionReq::parse("1.2.3").unwrap()),
                ))
                .collect::<BTreeMap<_, _>>(),
                defaults: None,
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
            iter::once(("cargo-liner".to_owned(), PackageRequirement::SIMPLE_STAR))
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
                ("cargo-liner".to_owned(), PackageRequirement::SIMPLE_STAR),
                (
                    "a".to_owned(),
                    PackageRequirement::Simple(VersionReq::parse("1.2.3").unwrap())
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
            PackageRequirement::Simple(VersionReq::parse("1.2.3").unwrap()),
        ))
        .collect::<BTreeMap<_, _>>();

        assert_eq!(
            UserConfig {
                packages: pkgs.clone(),
                defaults: None,
            }
            .update_others(false)
            .packages,
            pkgs,
        );
    }
}

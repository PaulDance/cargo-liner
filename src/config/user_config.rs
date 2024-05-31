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
                "#,
        )
        .unwrap()
        .packages
        .into_iter()
        .map(|(name, req)| {
            (
                name,
                (
                    req.version().to_string(),
                    req.features().to_owned(),
                    req.all_features(),
                    req.default_features(),
                    req.index().map(ToOwned::to_owned),
                    req.registry().map(ToOwned::to_owned),
                    req.extra_arguments().to_owned(),
                    req.environment(),
                ),
            )
        })
        .collect::<Vec<_>>();
        packages.sort_by_key(|(k, _)| k.clone());

        let packages: Vec<_> = packages.into_iter().map(|(_, v)| v).collect();

        let expected = [
            (
                "^1.2.3".to_owned(),
                vec![],
                false,
                true,
                None,
                None,
                vec![],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec!["foo".to_owned()],
                false,
                true,
                None,
                None,
                vec![],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec!["foo".to_owned()],
                false,
                false,
                None,
                None,
                vec![],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec![],
                true,
                true,
                None,
                None,
                vec![],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec![],
                false,
                true,
                None,
                None,
                vec![],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec![],
                false,
                true,
                None,
                None,
                vec![],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec![],
                false,
                true,
                None,
                None,
                vec!["--abc".to_owned(), "--def".to_owned()],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec![],
                false,
                true,
                None,
                None,
                vec![],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec![],
                false,
                true,
                None,
                None,
                vec![],
                [("ABC", "def"), ("XYZ", "123")]
                    .into_iter()
                    .map(|(k, v)| (k.to_owned(), v.to_owned()))
                    .collect(),
            ),
            (
                "^1.2".to_owned(),
                vec![],
                false,
                true,
                Some("http://abc123.com".to_owned()),
                None,
                vec![],
                BTreeMap::new(),
            ),
            (
                "^1.2".to_owned(),
                vec![],
                false,
                true,
                None,
                Some("abc123".to_owned()),
                vec![],
                BTreeMap::new(),
            ),
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

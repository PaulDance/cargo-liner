//! Mainly exports [`EffectiveConfig`]: a reunion of all configuration sources.
//!
//! This is a bit the role of crates such as `twelf`, simply re-done here to be
//! customized for the needs specific to the current project, which includes
//! parsing the various sources in split steps, doing it optionally, and also
//! sometimes only partially.

use std::collections::BTreeMap;

use super::{DetailedPackageReq, UserConfig};

/// Effective merge of all configuration sources.
#[derive(Debug)]
pub struct EffectiveConfig {
    /// The name-to-setting map derived from the [`UserConfig`].
    pub packages: BTreeMap<String, DetailedPackageReq>,
}

impl EffectiveConfig {
    /// Merges all given sources and exports the result as public fields.
    pub fn new(user_config: UserConfig) -> Self {
        Self {
            packages: user_config
                .packages
                .into_iter()
                .map(|(pkg_name, pkg)| (pkg_name, pkg.into()))
                .collect::<BTreeMap<String, DetailedPackageReq>>(),
        }
    }
}

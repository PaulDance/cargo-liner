use std::collections::BTreeMap;

use semver::VersionReq;
use serde::{Deserialize, Serialize};

/// Small helper function that returns true to work around
/// <https://github.com/serde-rs/serde/issues/368>.
#[allow(clippy::inline_always)]
#[inline(always)]
const fn serde_default_true() -> bool {
    true
}

/// Package requirement with additional options set.
///
/// See <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html>.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct DetailedPackageReq {
    // Options from the Cargo Install CLI.
    pub version: VersionReq,

    #[serde(default = "serde_default_true")]
    pub default_features: bool,

    #[serde(default)]
    pub all_features: bool,

    #[serde(default)]
    pub features: Vec<String>,

    #[serde(default)]
    pub index: Option<String>,

    #[serde(default)]
    pub registry: Option<String>,

    #[serde(default)]
    pub git: Option<String>,

    #[serde(default)]
    pub branch: Option<String>,

    #[serde(default)]
    pub tag: Option<String>,

    #[serde(default)]
    pub rev: Option<String>,

    #[serde(default)]
    pub path: Option<String>,

    #[serde(default)]
    pub bins: Vec<String>,

    #[serde(default)]
    pub all_bins: bool,

    #[serde(default)]
    pub examples: Vec<String>,

    #[serde(default)]
    pub all_examples: bool,

    #[serde(default)]
    pub force: bool,

    #[serde(default)]
    pub ignore_rust_version: bool,

    #[serde(default)]
    pub frozen: bool,

    #[serde(default)]
    pub locked: bool,

    #[serde(default)]
    pub offline: bool,

    // Additional options.
    /// Additional CLI arguments that must be passed onto the associated `cargo
    /// install` call between the last one set by proper options and the `--`
    /// separating the following fixed arguments.
    #[serde(default)]
    pub extra_arguments: Vec<String>,

    /// Environment variables that must be set for the `cargo install` process.
    #[serde(default)]
    pub environment: BTreeMap<String, String>,
}

// Should be kept in-sync with the above definition with regards to Serde.
impl Default for DetailedPackageReq {
    fn default() -> Self {
        Self {
            // Manual defaults.
            default_features: true,
            // Usual defaults.
            version: VersionReq::default(),
            all_features: bool::default(),
            features: Vec::default(),
            index: Option::default(),
            registry: Option::default(),
            git: Option::default(),
            branch: Option::default(),
            tag: Option::default(),
            rev: Option::default(),
            path: Option::default(),
            bins: Vec::default(),
            all_bins: bool::default(),
            examples: Vec::default(),
            all_examples: bool::default(),
            force: bool::default(),
            ignore_rust_version: bool::default(),
            frozen: bool::default(),
            locked: bool::default(),
            offline: bool::default(),
            extra_arguments: Vec::default(),
            environment: BTreeMap::default(),
        }
    }
}

impl From<PackageRequirement> for DetailedPackageReq {
    fn from(pkg: PackageRequirement) -> Self {
        match pkg {
            PackageRequirement::Simple(pkg_ver) => Self {
                version: pkg_ver,
                ..Default::default()
            },
            PackageRequirement::Detailed(det_pkg) => det_pkg,
        }
    }
}

/// Represents the requirement setting configured for a package.
///
/// The enumeration is deserialized from an untagged form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageRequirement {
    #[allow(clippy::doc_markdown)]
    /// Simple form: only a SemVer requirement string.
    Simple(VersionReq),
    /// Detailed form: all supported options made available.
    Detailed(DetailedPackageReq),
}

impl PackageRequirement {
    /// Convenience shortcut for simple and star version requirement package.
    pub const SIMPLE_STAR: Self = Self::Simple(VersionReq::STAR);
}

use semver::VersionReq;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct DetailedPackageReqSerdeHelper {
    version: VersionReq,
    #[serde(default)]
    features: Vec<String>,
    #[serde(default)]
    all_features: bool,
    #[serde(default)]
    no_default_features: bool,
}

// small helper function that returns true to work around https://github.com/serde-rs/serde/issues/368
fn serde_default_true() -> bool {
    true
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq)]
pub struct DetailedPackageReq {
    #[serde(rename = "default-features", default = "serde_default_true")]
    default_features: bool,

    #[serde(rename = "all-features", default)]
    all_features: bool,

    #[serde(default)]
    features: Vec<String>,

    version: VersionReq,
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
    Detailed(DetailedPackageReq),
}

impl Package {
    /// Convenience shortcut for simple and star version requirement package.
    pub const SIMPLE_STAR: Self = Self::Simple(VersionReq::STAR);

    pub fn version(&self) -> &VersionReq {
        match self {
            Package::Simple(v) => v,
            Package::Detailed(DetailedPackageReq { version, .. }) => version,
        }
    }

    pub fn all_features(&self) -> bool {
        match self {
            Package::Detailed(DetailedPackageReq {
                all_features: true, ..
            }) => true,
            _ => false,
        }
    }

    pub fn default_features(&self) -> bool {
        match self {
            Package::Detailed(DetailedPackageReq {
                default_features: false,
                ..
            }) => false,
            _ => true,
        }
    }

    pub fn features(&self) -> &[String] {
        match self {
            Package::Simple(_) => &[],
            Package::Detailed(DetailedPackageReq { features, .. }) => features.as_slice(),
        }
    }
}

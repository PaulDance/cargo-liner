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
    version: VersionReq,

    #[serde(default = "serde_default_true")]
    default_features: bool,

    #[serde(default)]
    all_features: bool,

    #[serde(default)]
    features: Vec<String>,

    #[serde(default)]
    index: Option<String>,

    #[serde(default)]
    registry: Option<String>,

    #[serde(default)]
    git: Option<String>,

    #[serde(default)]
    branch: Option<String>,

    #[serde(default)]
    tag: Option<String>,

    #[serde(default)]
    rev: Option<String>,

    #[serde(default)]
    path: Option<String>,

    #[serde(default)]
    bins: Vec<String>,

    #[serde(default)]
    all_bins: bool,

    #[serde(default)]
    examples: Vec<String>,

    #[serde(default)]
    all_examples: bool,

    #[serde(default)]
    force: bool,

    #[serde(default)]
    ignore_rust_version: bool,

    #[serde(default)]
    frozen: bool,

    #[serde(default)]
    locked: bool,

    #[serde(default)]
    offline: bool,

    // Additional options.
    /// Additional CLI arguments that must be passed onto the associated `cargo
    /// install` call between the last one set by proper options and the `--`
    /// separating the following fixed arguments.
    #[serde(default)]
    extra_arguments: Vec<String>,

    /// Environment variables that must be set for the `cargo install` process.
    #[serde(default)]
    environment: BTreeMap<String, String>,
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

    /// Returns the version requirement of the package.
    pub fn version(&self) -> &VersionReq {
        match self {
            Self::Simple(v) => v,
            Self::Detailed(pkg_req) => &pkg_req.version,
        }
    }

    /// Returns whether or not all features were required for the package,
    /// defaulting to `false` when the package is simple.
    pub fn all_features(&self) -> bool {
        matches!(
            self,
            Self::Detailed(DetailedPackageReq {
                all_features: true,
                ..
            })
        )
    }

    /// Returns whether or not default features were required for the package,
    /// defaulting to `true` when the package is simple.
    pub fn default_features(&self) -> bool {
        !matches!(
            self,
            Self::Detailed(DetailedPackageReq {
                default_features: false,
                ..
            })
        )
    }

    /// Returns a slice of the feature names required for the package,
    /// defaulting to an empty one when the package is simple.
    pub fn features(&self) -> &[String] {
        match self {
            Self::Simple(_) => &[],
            Self::Detailed(pkg_req) => pkg_req.features.as_slice(),
        }
    }

    /// Returns the index to use or `None` if either not configured or simple.
    pub fn index(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(pkg_req) => pkg_req.index.as_deref(),
        }
    }

    /// Returns the registry to use or `None` if either not configured or
    /// simple.
    pub fn registry(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(pkg_req) => pkg_req.registry.as_deref(),
        }
    }

    /// Returns the Git URL to use or `None` if either not configured or simple.
    pub fn git(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(pkg_req) => pkg_req.git.as_deref(),
        }
    }

    /// Returns the Git branch to use or `None` if either not configured or
    /// simple.
    pub fn branch(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(pkg_req) => pkg_req.branch.as_deref(),
        }
    }

    /// Returns the Git tag to use or `None` if either not configured or simple.
    pub fn tag(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(pkg_req) => pkg_req.tag.as_deref(),
        }
    }

    /// Returns the Git commit to use or `None` if either not configured or
    /// simple.
    pub fn rev(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(pkg_req) => pkg_req.rev.as_deref(),
        }
    }

    /// Returns the file path to use or `None` if either not configured or
    /// simple.
    pub fn path(&self) -> Option<&str> {
        match self {
            Self::Simple(_) => None,
            Self::Detailed(pkg_req) => pkg_req.path.as_deref(),
        }
    }

    /// Returns the list of binary targets to install, empty if simple.
    pub fn bins(&self) -> &[String] {
        match self {
            Self::Simple(_) => &[],
            Self::Detailed(pkg_req) => &pkg_req.bins,
        }
    }

    /// Returns `true` iff the package is detailed and `all-bins` was passed as
    /// `true`.
    pub fn all_bins(&self) -> bool {
        matches!(
            self,
            Self::Detailed(DetailedPackageReq { all_bins: true, .. })
        )
    }

    /// Returns the list of example targets to install, empty if simple.
    pub fn examples(&self) -> &[String] {
        match self {
            Self::Simple(_) => &[],
            Self::Detailed(pkg_req) => &pkg_req.examples,
        }
    }

    /// Returns `true` iff the package is detailed and `all-examples` was
    /// passed as `true`.
    pub fn all_examples(&self) -> bool {
        matches!(
            self,
            Self::Detailed(DetailedPackageReq {
                all_examples: true,
                ..
            })
        )
    }

    /// Returns `true` iff the package is detailed and `force` was passed as
    /// `true`.
    pub fn force(&self) -> bool {
        matches!(self, Self::Detailed(DetailedPackageReq { force: true, .. }))
    }

    /// Returns `true` iff the package is detailed and `ignore-rust-version` was
    /// passed as `true`.
    pub fn ignore_rust_version(&self) -> bool {
        matches!(
            self,
            Self::Detailed(DetailedPackageReq {
                ignore_rust_version: true,
                ..
            })
        )
    }

    /// Returns `true` iff the package is detailed and `frozen` was passed as
    /// `true`.
    pub fn frozen(&self) -> bool {
        matches!(
            self,
            Self::Detailed(DetailedPackageReq { frozen: true, .. })
        )
    }

    /// Returns `true` iff the package is detailed and `locked` was passed as
    /// `true`.
    pub fn locked(&self) -> bool {
        matches!(
            self,
            Self::Detailed(DetailedPackageReq { locked: true, .. })
        )
    }

    /// Returns `true` iff the package is detailed and `offline` was passed as
    /// `true`.
    pub fn offline(&self) -> bool {
        matches!(
            self,
            Self::Detailed(DetailedPackageReq { offline: true, .. })
        )
    }

    /// Returns a slice of the extra `cargo install` arguments required for the
    /// package, defaulting to an empty one when the package is simple.
    pub fn extra_arguments(&self) -> &[String] {
        match self {
            Self::Simple(_) => &[],
            Self::Detailed(pkg_req) => pkg_req.extra_arguments.as_slice(),
        }
    }

    /// Returns a clone of the environment variables that should be set for the
    /// spawned `cargo install` process, defaulting to an empty map when the
    /// package is simple.
    pub fn environment(&self) -> BTreeMap<String, String> {
        match self {
            Self::Simple(_) => BTreeMap::new(),
            Self::Detailed(pkg_req) => pkg_req.environment.clone(),
        }
    }
}

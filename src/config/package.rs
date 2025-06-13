use std::collections::BTreeMap;

use semver::VersionReq;
use serde::{Deserialize, Serialize};

use crate::cli::BinstallChoice;

/// Small helper function that returns true to work around
/// <https://github.com/serde-rs/serde/issues/368>.
#[expect(
    clippy::inline_always,
    reason = "A simple literal is a perfect candidate for inlining."
)]
#[inline(always)]
const fn serde_default_true() -> bool {
    true
}

/// Package requirement with additional options set.
///
/// See <https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html>.
#[expect(
    clippy::struct_excessive_bools,
    reason = "This is configuration, so needs to represent all possible items, which includes flags."
)]
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

    /// Do the same as the global `--skip-check` but only for this package.
    #[serde(default)]
    pub skip_check: bool,

    /// Do the same as the global `--no-fail-fast` but only for this package.
    #[serde(default)]
    pub no_fail_fast: bool,

    /// Do the same as the global `--binstall` but only for this package.
    #[serde(default)]
    pub binstall: Option<BinstallChoice>,
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
            skip_check: bool::default(),
            no_fail_fast: bool::default(),
            binstall: Option::default(),
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
            PackageRequirement::Detailed(det_pkg) => *det_pkg,
        }
    }
}

impl DetailedPackageReq {
    /// Returns whether the package should be version-checked beyond only the
    /// raw `skip_check` element by taking other fields into consideration.
    pub fn effective_skip_check(&self) -> bool {
        self.skip_check
            || self.path.is_some()
            || self.git.is_some()
            || self.branch.is_some()
            || self.tag.is_some()
            || self.rev.is_some()
    }
}

/// Represents the requirement setting configured for a package.
///
/// The enumeration is deserialized from an untagged form.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PackageRequirement {
    #[expect(
        clippy::doc_markdown,
        reason = "SemVer is not a Rust type here, but a proper noun."
    )]
    /// Simple form: only a SemVer requirement string.
    Simple(VersionReq),
    /// Detailed form: all supported options made available.
    Detailed(Box<DetailedPackageReq>),
}

impl PackageRequirement {
    /// Convenience shortcut for simple and star version requirement package.
    pub const SIMPLE_STAR: Self = Self::Simple(VersionReq::STAR);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_default() {
        assert!(!DetailedPackageReq::default().effective_skip_check());
    }

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_skipcheck() {
        assert!(
            DetailedPackageReq {
                skip_check: true,
                ..Default::default()
            }
            .effective_skip_check()
        );
    }

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_path() {
        assert!(
            DetailedPackageReq {
                path: Some("/a/b/c".to_owned()),
                ..Default::default()
            }
            .effective_skip_check()
        );
    }

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_git() {
        assert!(
            DetailedPackageReq {
                git: Some("ssh://git@example.com/user/repo.git".to_owned()),
                ..Default::default()
            }
            .effective_skip_check()
        );
    }

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_branch() {
        assert!(
            DetailedPackageReq {
                branch: Some("example".to_owned()),
                ..Default::default()
            }
            .effective_skip_check()
        );
    }

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_tag() {
        assert!(
            DetailedPackageReq {
                tag: Some("example".to_owned()),
                ..Default::default()
            }
            .effective_skip_check()
        );
    }

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_rev() {
        assert!(
            DetailedPackageReq {
                rev: Some("1337133713371337133713371337133713371337".to_owned()),
                ..Default::default()
            }
            .effective_skip_check()
        );
    }

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_combination_1() {
        assert!(
            DetailedPackageReq {
                skip_check: true,
                path: Some("/a/b/c".to_owned()),
                ..Default::default()
            }
            .effective_skip_check()
        );
    }

    #[test]
    fn test_detailedpackagereq_effectiveskipcheck_combination_2() {
        assert!(
            DetailedPackageReq {
                git: Some("ssh://git@example.com/user/repo.git".to_owned()),
                branch: Some("example".to_owned()),
                ..Default::default()
            }
            .effective_skip_check()
        );
    }
}

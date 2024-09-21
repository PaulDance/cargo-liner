//! Module handling all the user configuration deserializing and validating.
//!
//! See [`UserConfig::parse_file`] in order to retrieve such configuration
//! settings from the default file.
#![expect(
    clippy::module_name_repetitions,
    reason = "Sub-parts of the global configuration are still configuration in themselves."
)]

mod cargo_crates_toml;
mod effective_config;
pub mod env;
mod package;
mod user_config;

pub use cargo_crates_toml::CargoCratesToml;
pub use effective_config::EffectiveConfig;
pub use package::{DetailedPackageReq, PackageRequirement};
pub use user_config::UserConfig;

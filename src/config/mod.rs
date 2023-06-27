//! Module handling all the user configuration deserializing and validating.
//!
//! See [`UserConfig::parse_file`] in order to retrieve such configuration
//! settings from the default file.
#![allow(clippy::module_name_repetitions)]

mod cargo_crates_toml;
mod package;
mod user_config;

pub use cargo_crates_toml::CargoCratesToml;
pub use package::Package;
pub use user_config::UserConfig;

use std::{env, fs};

use cargo_test_macro::cargo_test;
use cargo_test_support::registry::Package;
use trycmd::TestCases;

mod common;
use common::init_registry;

/// Verify that all `console` code blocks included in this project's
/// `README.md` file are properly kept in-sync with the actual outputs.
///
/// Use `TRYCMD=overwrite cargo test validate_readme` in order to automatically
/// update each code block's contents.
#[cargo_test]
fn validate_readme() {
    // Add example packages to registry.
    let _reg = init_registry();
    Package::new("cargo-liner", "0.0.0")
        .file("src/main.rs", "fn main() {}")
        .publish();
    Package::new("bat", "0.24.0")
        .file("src/main.rs", "fn main() {}")
        .publish();
    Package::new("cargo-expand", "1.0.79")
        .file("src/main.rs", "fn main() {}")
        .publish();

    // Retrieve some test paths.
    let tmp_home = cargo_test_support::paths::home();
    let tmp_cargo_home = tmp_home.join(".cargo");
    let tmp_cargo_home_bin = tmp_cargo_home.join("bin");

    // Mimic previous `cargo install` runs.
    fs::create_dir(tmp_cargo_home_bin.clone()).unwrap();
    fs::write(tmp_cargo_home_bin.join("cargo-liner"), "").unwrap();
    fs::write(tmp_cargo_home_bin.join("cargo-expand"), "").unwrap();
    fs::write(
        tmp_cargo_home.join(".crates.toml"),
        [
            "[v1]",
            "\"cargo-liner 0.0.0 (registry+https://github.com/rust-lang/crates.io-index)\" = [\"cargo-liner\"]",
            "\"cargo-expand 1.0.78 (registry+https://github.com/rust-lang/crates.io-index)\" = [\"cargo-expand\"]",
        ]
        .join("\n"),
    )
    .unwrap();
    // Add our example configuration.
    fs::write(
        tmp_cargo_home.join("liner.toml"),
        ["[packages]", "bat = \"*\"", "cargo-expand = \"*\""].join("\n"),
    )
    .unwrap();

    // Run the test.
    TestCases::new()
        .register_bin("cargo", trycmd::cargo::cargo_bin!(clap::crate_name!()))
        // Mimic part of Cargo's test support.
        .env("HOME", tmp_home.to_str().unwrap())
        .env("CARGO_HOME", tmp_cargo_home.to_str().unwrap())
        .env(
            "__CARGO_TEST_ROOT",
            cargo_test_support::paths::global_root().to_str().unwrap(),
        )
        // Avoid unnecessary additional data.
        .env("CARGO_INCREMENTAL", "0")
        // Restrict the output's width by emulating a narrower terminal in
        // order for the file to be kept properly-formatted. 70 columns should
        // make the output fit in the current rendering of crates.io for code
        // blocks.
        .env("COLUMNS", "70")
        .case("README.md")
        .run();
}

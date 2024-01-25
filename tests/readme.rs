use std::{env, fs};

use cargo_test_macro::cargo_test;
use cargo_test_support::registry::Package;
use trycmd::TestCases;

mod common;
use common::{fake_install_all, init_registry};

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

    // Mimic previous `cargo install` runs.
    fake_install_all([("cargo-liner", "0.0.0"), ("cargo-expand", "1.0.78")]);
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

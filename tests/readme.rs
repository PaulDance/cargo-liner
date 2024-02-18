use cargo_test_macro::cargo_test;
use trycmd::TestCases;

mod common;
use common::*;

/// Verify that all `console` code blocks included in this project's
/// `README.md` file are properly kept in-sync with the actual outputs.
///
/// Use `TRYCMD=overwrite cargo test validate_readme` in order to automatically
/// update each code block's contents.
#[cargo_test]
fn validate_readme() {
    // Add example packages to registry.
    let _reg = init_registry();
    fake_publish_all([
        ("cargo-liner", "0.0.0", false),
        ("bat", "0.24.0", false),
        ("cargo-expand", "1.0.79", false),
    ]);

    // Retrieve some test paths.
    let tmp_home = cargo_test_support::paths::home();
    let tmp_cargo_home = tmp_home.join(".cargo");

    // Mimic previous `cargo install` runs.
    fake_install_all([
        ("cargo-liner", "0.0.0", false),
        ("cargo-expand", "1.0.78", false),
    ]);
    // Add our example configuration.
    write_user_config(&["[packages]", "bat = '*'", "cargo-expand = '*'"]);

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
        .env("COLUMNS", "69")
        .case("README.md")
        .run();
}

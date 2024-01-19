use trycmd::TestCases;

/// Verify that all `console` code blocks included in this project's
/// `README.md` file are properly kept in-sync with the actual outputs.
///
/// Use `TRYCMD=overwrite cargo test validate_readme` in order to automatically
/// update each code block's contents.
#[test]
fn validate_readme() {
    TestCases::new()
        .register_bin("cargo", trycmd::cargo::cargo_bin!(clap::crate_name!()))
        // Restrict the output's width by emulating a narrower terminal in
        // order for the file to be kept properly-formatted. 70 columns should
        // make the output fit in the current rendering of crates.io for code
        // blocks.
        .env("COLUMNS", "70")
        .case("README.md")
        .run();
}

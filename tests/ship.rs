use cargo_test_macro::cargo_test;
use cargo_test_support::registry::Package;
use indoc::indoc;
use snapbox::IntoData;
use tempfile::TempDir;

mod common;
use common::*;

#[cargo_test]
fn validate_ship_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .arg("ship")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_onlyself_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--only-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_noself_nothing() {
    let _reg = init_registry();
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_skipcheck_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_noupdate.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_skipcheck_onlyself_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--only-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_noupdate.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_skipcheck_noself_nothing() {
    let _reg = init_registry();
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(
            snapbox::file!["fixtures/ship/validate_ship_skipcheck_noself_nothing.stderr"].raw(),
        );
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_force_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_onlyself_force_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--only-self", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_noself_force_nothing() {
    let _reg = init_registry();
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--no-self", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_skipcheck_force_update() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_force_update.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_skipcheck_onlyself_force_update() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--only-self", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_force_update.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_skipcheck_noself_force_nothing() {
    let _reg = init_registry();
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(
            snapbox::file!["fixtures/ship/validate_ship_skipcheck_noself_nothing.stderr"].raw(),
        );
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_newerself_update() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.4");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .arg("ship")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_newerself_update.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_newerself_onlyself_update() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.4");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--only-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_newerself_update.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_newerself_noself_nothing() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.4");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_newerself_skipcheck_update() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.4");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_newerself_skipcheck_update.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_newerself_skipcheck_onlyself_update() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.4");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--only-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_newerself_skipcheck_update.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_newerself_skipcheck_noself_nothing() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.4");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(
            snapbox::file!["fixtures/ship/validate_ship_skipcheck_noself_nothing.stderr"].raw(),
        );
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_manynotinstalled_install() {
    let _reg = init_registry();
    fixture_fake_publish();
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    fixture_write_user_config();

    cargo_liner()
        .arg("ship")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_manynotinstalled_install.stderr"
        ]);
    fixture_assert_installed();
}

#[cargo_test]
fn validate_ship_manyinstalled_noupdate() {
    let _reg = init_registry();
    fixture_fake_install();
    fixture_assert_installed();
    fixture_fake_publish();
    fixture_write_user_config();

    cargo_liner()
        .arg("ship")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_manyinstalled_noupdate.stderr"
        ]);
    fixture_assert_installed();
}

#[cargo_test]
fn validate_ship_manynewer_update() {
    let _reg = init_registry();
    fixture_fake_install();
    fixture_assert_installed();
    fixture_fake_publish();
    fixture_fake_publish_newer_others();
    fixture_write_user_config();

    cargo_liner()
        .arg("ship")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_manynewer_update.stderr"
        ]);
    fixture_assert_installed();
}

/// Checks that configuration can indeed apply to ourselves too.
#[cargo_test]
fn validate_ship_selfupdate_customization() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    // Use `--force` for simplicity: just checking the flag is passed is enough.
    write_user_config(&[
        "[packages]",
        "cargo-liner = { version = '*', force = true }",
    ]);

    cargo_liner()
        .args(["ship", "--skip-check", "--only-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_force_update.stderr"
        ]);
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_features_simple_none() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("feat", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(feature = "feat")]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_simple_default() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("default", &["feat"])
        .feature("feat", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(not(feature = "feat"))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_none() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("feat", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(feature = "feat")]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&["[packages]", "pkg = { version = '*' }"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_one() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("feat", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(not(feature = "feat"))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&["[packages]", "pkg = { version = '*', features = ['feat'] }"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_default() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("default", &["feat"])
        .feature("feat", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(not(feature = "feat"))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&["[packages]", "pkg = { version = '*' }"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_nodefault() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("default", &["feat"])
        .feature("feat", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(feature = "feat")]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&[
        "[packages]",
        "pkg = { version = '*', default-features = false }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_nodefault_one() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("default", &["feat1"])
        .feature("feat1", &[])
        .feature("feat2", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(any(
                    feature = "feat1",
                    not(feature = "feat2"),
                ))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&[
        "[packages]",
        "pkg = { version = '*', default-features = false, features = ['feat2'] }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_all() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("feat1", &[])
        .feature("feat2", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(any(
                    not(feature = "feat1"),
                    not(feature = "feat2"),
                ))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&["[packages]", "pkg = { version = '*', all-features = true }"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_all_one() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("feat1", &[])
        .feature("feat2", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(any(
                    not(feature = "feat1"),
                    not(feature = "feat2"),
                ))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&[
        "[packages]",
        "pkg = { version = '*', all-features = true, features = ['feat1'] }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_all_default() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("default", &["feat1"])
        .feature("feat1", &[])
        .feature("feat2", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(any(
                    not(feature = "feat1"),
                    not(feature = "feat2"),
                ))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&[
        "[packages]",
        "pkg = { version = '*', all-features = true, default-features = true }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_all_nodefault() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("default", &["feat1"])
        .feature("feat1", &[])
        .feature("feat2", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(any(
                    not(feature = "feat1"),
                    not(feature = "feat2"),
                ))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&[
        "[packages]",
        "pkg = { version = '*', all-features = true, default-features = false }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_all_default_one() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("default", &["feat1"])
        .feature("feat1", &[])
        .feature("feat2", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(any(
                    not(feature = "feat1"),
                    not(feature = "feat2"),
                ))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&[
        "[packages.pkg]",
        "version = '*'",
        "all-features = true",
        "default-features = true",
        "features = ['feat2']",
    ]);
    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_features_detailed_all_nodefault_one() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("pkg", "0.0.0")
        .feature("default", &["feat1"])
        .feature("feat1", &[])
        .feature("feat2", &[])
        .file(
            "src/main.rs",
            r#"
                #[cfg(any(
                    not(feature = "feat1"),
                    not(feature = "feat2"),
                ))]
                compile_error!("");
                fn main() {}
            "#,
        )
        .publish();
    write_user_config(&[
        "[packages.pkg]",
        "version = '*'",
        "all-features = true",
        "default-features = false",
        "features = ['feat2']",
    ]);
    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_verbosity_v() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "0.0.0");
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["-v", "ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_verbosity_v.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_verbosity_vv() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "0.0.0");
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["-vv", "ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_verbosity_vv.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_verbosity_vvv() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "0.0.0");
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["-vvv", "ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_verbosity_vvv.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_verbosity_q() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "0.0.0");
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["-q", "ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_verbosity_q.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_verbosity_qq() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "0.0.0");
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["-qq", "ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_verbosity_q.stderr"
        ]);
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_verbosity_qqq() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "0.0.0");
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["-qqq", "ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(
            indoc!(
                "
                    Updating `dummy-registry` index
                note: to learn more about a package, run `cargo info <name>`
                "
            )
            .raw(),
        );
    assert_installed("pkg");
}

#[cargo_test]
fn validate_ship_noconfig_iserr() {
    let _reg = init_registry();
    fake_install_self();

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_noconfig_iserr.stderr"
        ]);
}

/// See #5.
#[cargo_test]
fn validate_ship_nocratestoml_iserr() {
    let _reg = init_registry();
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_nocratestoml_iserr.stderr"
        ]);
}

/// See #5.
#[cargo_test]
fn validate_ship_nocratestoml_skipcheck_isok() {
    let _reg = init_registry();
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(
            snapbox::file!["fixtures/ship/validate_ship_skipcheck_noself_nothing.stderr"].raw(),
        );
}

/// See #4.
#[cargo_test]
fn validate_ship_cargoinstallroot_supported() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "0.0.0");
    write_user_config(&["[packages]", "pkg = '*'"]);

    let tmp_dir = TempDir::new().unwrap();
    cargo_liner()
        .env("CARGO_INSTALL_ROOT", tmp_dir.path())
        .args(["ship", "--no-self", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_cargoinstallroot_supported.stderr"
        ]);
    assert!(
        tmp_dir
            .path()
            .join("bin")
            .join(cargo_test_support::install::exe("pkg"))
            .is_file()
    );
}

/// See #6
#[cargo_test]
fn validate_ship_weirdversions_supported() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "1.0.0+meta123");
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_weirdversions_supported.stderr"
        ]);
    assert_installed("pkg");
}

/// See #7
#[cargo_test]
fn validate_ship_weirdpackagenames_supported() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("--pkg", "0.0.0");
    write_user_config(&["[packages]", "--pkg = '*'"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_weirdpackagenames_supported.stderr"
        ]);
    assert_not_installed("--pkg");
}

/// See #14.
#[cargo_test]
fn validate_ship_cargotermcolor_supported() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("pkg", "0.0.0");
    write_user_config(&["[packages]", "pkg = '*'"]);

    cargo_liner()
        .env("CARGO_TERM_COLOR", "always")
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

/// See #11.
#[cargo_test]
fn validate_ship_install_failfast() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("def", "0.0.0");
    write_user_config(&["[packages]", "abc = '*'", "def = '*'"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_install_failfast.stderr"
        ]);
    assert_not_installed_all(["abc", "def"]);
}

#[cargo_test]
fn validate_ship_install_nofailfast_err_iserr() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("def", "0.0.0");
    write_user_config(&["[packages]", "abc = '*'", "def = '*'"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self", "--no-fail-fast"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_install_nofailfast_err_iserr.stderr"
        ]);
    assert_not_installed("abc");
    assert_installed("def");
}

#[cargo_test]
fn validate_ship_install_nofailfast_ok_isok() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([("abc", "0.0.0"), ("def", "0.0.0")]);
    write_user_config(&["[packages]", "abc = '*'", "def = '*'"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self", "--no-fail-fast"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_install_nofailfast_ok_isok.stderr"
        ]);
    assert_installed("abc");
    assert_installed("def");
}

/// See #23.
#[cargo_test]
fn validate_ship_extrargs_list() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    write_user_config(&[
        "[packages]",
        // HACK: use `--list` in order to obtain a small and stable output
        // sufficient to confirm the argument is taken into account.
        "abc = { version = '*', extra-arguments = ['--list'] }",
    ]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self"])
        .assert()
        .success()
        .stdout_eq(snapbox::file![
            "fixtures/ship/validate_ship_extrargs_list.stdout"
        ])
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_extrargs_list.stderr"].raw());
}

/// See #23.
#[cargo_test]
fn validate_ship_environment_list() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    write_user_config(&[
        "[packages]",
        // HACK: use the quiet control to observe a stable change in behavior.
        "abc = { version = '*', environment = { CARGO_TERM_QUIET = 'true' } }",
    ]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_extrargs_list.stderr"].raw());
    assert_installed("abc");
}

#[cargo_test]
fn validate_ship_index() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', index = '' }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_index.stderr"]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_registry() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', registry = '' }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_registry.stderr"
        ]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_git() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', git = '' }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        // Fails on the same URL parsing error.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_index.stderr"]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_branch() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', branch = '' }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_branch.stderr"]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_tag() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', tag = '' }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_tag.stderr"]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_rev() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', rev = '' }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_rev.stderr"]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_path() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', path = '/a/b/c' }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_path.stderr"]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_bins() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("abc", "0.0.0")
        .file("src/bin/b1.rs", "fn main() {}")
        .file("src/bin/b2.rs", "fn main() {}")
        .file("src/bin/b3.rs", "fn main() {}")
        .publish();
    write_user_config(&["[packages]", "abc = { version = '*', bins = ['b1', 'b2'] }"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_bins.stderr"]);
    assert_installed_all(["b1", "b2"]);
    assert_not_installed("b3");
}

#[cargo_test]
fn validate_ship_all_bins() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("abc", "0.0.0")
        .file("src/bin/b1.rs", "fn main() {}")
        .file("src/bin/b2.rs", "fn main() {}")
        .file("src/bin/b3.rs", "fn main() {}")
        .publish();
    write_user_config(&["[packages]", "abc = { version = '*', all-bins = true }"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_all_bins.stderr"
        ]);
    assert_installed_all(["b1", "b2", "b3"]);
}

#[cargo_test]
fn validate_ship_examples() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("abc", "0.0.0")
        .file("src/lib.rs", "")
        .file("examples/ex1.rs", "fn main() {}")
        .file("examples/ex2.rs", "fn main() {}")
        .file("examples/ex3.rs", "fn main() {}")
        .publish();
    write_user_config(&[
        "[packages]",
        "abc = { version = '*', examples = ['ex1', 'ex2'] }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_examples.stderr"
        ]);
    assert_installed_all(["ex1", "ex2"]);
    assert_not_installed("ex3");
}

#[cargo_test]
fn validate_ship_all_examples() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("abc", "0.0.0")
        .file("src/lib.rs", "")
        .file("examples/ex1.rs", "fn main() {}")
        .file("examples/ex2.rs", "fn main() {}")
        .file("examples/ex3.rs", "fn main() {}")
        .publish();
    write_user_config(&["[packages]", "abc = { version = '*', all-examples = true }"]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_all_examples.stderr"
        ]);
    assert_installed_all(["ex1", "ex2", "ex3"]);
}

#[cargo_test]
fn validate_ship_skipcheck_configforce() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]", "abc = { version = '*', force = true }"]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_configforce.stderr"
        ]);
    assert_installed("abc");
}

#[cargo_test]
fn validate_ship_ignorerustversion() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("abc", "0.0.0")
        .file("src/main.rs", "fn main() {}")
        // Hopefully, that's enough.
        .rust_version("999.999")
        .publish();
    write_user_config(&[
        "[packages]",
        "abc = { version = '*', ignore-rust-version = true }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_ignorerustversion.stderr"
        ]);
    assert_installed("abc");
}

#[cargo_test]
fn validate_ship_frozen() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    // Instead of a message naming the parameter, now Cargo fails on finding
    // the package as if it did not exist because it will not download info.
    write_user_config(&["[packages]", "abc = { version = '*', frozen = true }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_frozen.stderr"]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_locked() {
    let _reg = init_registry();
    fake_install_self();
    Package::new("abc", "0.0.0")
        .file("src/main.rs", "fn main() {}")
        .dep("def", "0.0.0")
        .publish();
    Package::new("def", "0.0.0")
        .file("src/lib.rs", "")
        .publish();
    // HACK: observe the warning to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', locked = true }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_locked.stderr"]);
    assert_installed("abc");
}

#[cargo_test]
fn validate_ship_offline() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    // HACK: observe the error to confirm the argument is passed.
    write_user_config(&["[packages]", "abc = { version = '*', offline = true }"]);

    cargo_liner()
        .args(["-q", "ship", "--skip-check", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_offline.stderr"]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_ship_partial_skipcheck() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([
        ("p1", "0.0.0"),
        ("p2", "0.0.0"),
        ("p3", "0.0.1"),
        ("p4", "0.0.0"),
        ("p5", "0.0.0"),
        ("p6", "0.0.1"),
    ]);
    fake_install_all([
        ("p2", "0.0.0", false),
        ("p3", "0.0.0", false),
        ("p5", "0.0.0", false),
        ("p6", "0.0.0", false),
    ]);
    write_user_config(&[
        "[packages]",
        // Verify normal cases with the version check enabled still work
        // unchanged even when other packages have the check disabled.
        "p1 = '*'",
        "p2 = '*'",
        "p3 = '*'",
        // Not yet installed: install to perform.
        "p4 = { version = '*', skip-check = true }",
        // Installed and up-to-date: nothing to do, but still perform install
        // that ends up doing nothing since `skip-check` is used.
        "p5 = { version = '*', skip-check = true }",
        // Installed and not up-to-date: perform update.
        "p6 = { version = '*', skip-check = true }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_partial_skipcheck.stderr"
        ]);
    assert_installed_all(["p1", "p2", "p3", "p4", "p5", "p6"]);
}

/// Same cases at the previous test, but with `--skip-check` globally given in
/// order to assert it has priority over the package-local equivalent.
#[cargo_test]
fn validate_ship_partialandglobal_skipcheck() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([
        ("p1", "0.0.0"),
        ("p2", "0.0.0"),
        ("p3", "0.0.1"),
        ("p4", "0.0.0"),
        ("p5", "0.0.0"),
        ("p6", "0.0.1"),
    ]);
    fake_install_all([
        ("p2", "0.0.0", false),
        ("p3", "0.0.0", false),
        ("p5", "0.0.0", false),
        ("p6", "0.0.0", false),
    ]);
    write_user_config(&[
        "[packages]",
        // Verify normal cases with the version check enabled still work
        // unchanged even when other packages have the check disabled.
        "p1 = '*'",
        "p2 = '*'",
        "p3 = '*'",
        // Not yet installed: install to perform.
        "p4 = { version = '*', skip-check = true }",
        // Installed and up-to-date: nothing to do, but still perform install
        // that ends up doing nothing since `skip-check` is used.
        "p5 = { version = '*', skip-check = true }",
        // Installed and not up-to-date: perform update.
        "p6 = { version = '*', skip-check = true }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_partialandglobal_skipcheck.stderr"
        ]);
    assert_installed_all(["p1", "p2", "p3", "p4", "p5", "p6"]);
}

#[cargo_test]
fn validate_ship_partial_nofailfast_allerrnoff_isenderr() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([("p1", "0.0.0"), ("p3", "0.0.0"), ("p5", "0.0.0")]);
    write_user_config(&[
        "[packages]",
        "p1 = '*'",
        "p2 = { version = '*', no-fail-fast = true }",
        "p3 = '*'",
        "p4 = { version = '*', no-fail-fast = true }",
        "p5 = '*'",
    ]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_partial_nofailfast_allerrnoff_isenderr.stderr"
        ]);
    assert_installed_all(["p1", "p3", "p5"]);
    assert_not_installed_all(["p2", "p4"]);
}

#[cargo_test]
fn validate_ship_partial_nofailfast_oneerrnoff_oneerrff_isfasterr() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([("p1", "0.0.0"), ("p3", "0.0.0"), ("p5", "0.0.0")]);
    write_user_config(&[
        "[packages]",
        "p1 = '*'",
        "p2 = { version = '*', no-fail-fast = true }",
        "p3 = '*'",
        "p4 = { version = '*', no-fail-fast = false }",
        "p5 = '*'",
    ]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_partial_nofailfast_oneerrnoff_oneerrff_isfasterr.stderr"
        ]);
    assert_installed_all(["p1", "p3"]);
    assert_not_installed_all(["p2", "p4", "p5"]);
}

#[cargo_test]
fn validate_ship_partial_nofailfast_allok_isok() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([
        ("p1", "0.0.0"),
        ("p2", "0.0.0"),
        ("p3", "0.0.0"),
        ("p4", "0.0.0"),
        ("p5", "0.0.0"),
    ]);
    write_user_config(&[
        "[packages]",
        "p1 = '*'",
        "p2 = { version = '*', no-fail-fast = true }",
        "p3 = '*'",
        "p4 = { version = '*', no-fail-fast = true }",
        "p5 = '*'",
    ]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_partial_nofailfast_allok_isok.stderr"
        ]);
    assert_installed_all(["p1", "p2", "p3", "p4", "p5"]);
}

// Same as the above, but with the option globally enabled in order to check it
// has precedence.
#[cargo_test]
fn validate_ship_partialandglobal_nofailfast_allerrnoff_isenderr() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([("p1", "0.0.0"), ("p3", "0.0.0"), ("p5", "0.0.0")]);
    write_user_config(&[
        "[packages]",
        "p1 = '*'",
        "p2 = { version = '*', no-fail-fast = true }",
        "p3 = '*'",
        "p4 = { version = '*', no-fail-fast = true }",
        "p5 = '*'",
    ]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check", "--no-fail-fast"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_partialandglobal_nofailfast_allerrnoff_isenderr.stderr"
        ]);
    assert_installed_all(["p1", "p3", "p5"]);
    assert_not_installed_all(["p2", "p4"]);
}

#[cargo_test]
fn validate_ship_partialandglobal_nofailfast_oneerrnoff_oneerrff_isenderr() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([("p1", "0.0.0"), ("p3", "0.0.0"), ("p5", "0.0.0")]);
    write_user_config(&[
        "[packages]",
        "p1 = '*'",
        "p2 = { version = '*', no-fail-fast = true }",
        "p3 = '*'",
        "p4 = { version = '*', no-fail-fast = false }",
        "p5 = '*'",
    ]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check", "--no-fail-fast"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_partialandglobal_nofailfast_oneerrnoff_oneerrff_isenderr.stderr"
        ]);
    assert_installed_all(["p1", "p3", "p5"]);
    assert_not_installed_all(["p2", "p4"]);
}

#[cargo_test]
fn validate_ship_partialandglobal_nofailfast_allok_isok() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([
        ("p1", "0.0.0"),
        ("p2", "0.0.0"),
        ("p3", "0.0.0"),
        ("p4", "0.0.0"),
        ("p5", "0.0.0"),
    ]);
    write_user_config(&[
        "[packages]",
        "p1 = '*'",
        "p2 = { version = '*', no-fail-fast = true }",
        "p3 = '*'",
        "p4 = { version = '*', no-fail-fast = true }",
        "p5 = '*'",
    ]);

    cargo_liner()
        .args(["ship", "--no-self", "--skip-check", "--no-fail-fast"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            // Same as above.
            "fixtures/ship/validate_ship_partial_nofailfast_allok_isok.stderr"
        ]);
    assert_installed_all(["p1", "p2", "p3", "p4", "p5"]);
}

#[cargo_test]
fn validate_ship_cliwithself_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["ship", "--with-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_envnoself_nothing() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .env("CARGO_LINER_SHIP_NO_SELF", "true")
        .arg("ship")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_envnoself_cliwithself_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]"]);

    cargo_liner()
        .env("CARGO_LINER_SHIP_NO_SELF", "true")
        .args(["ship", "--with-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_confignoself_nothing() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]", "[defaults]", "ship.no-self = true"]);

    cargo_liner()
        .arg("ship")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_confignoself_envwithself_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]", "[defaults]", "ship.no-self = true"]);

    cargo_liner()
        .env("CARGO_LINER_SHIP_NO_SELF", "false")
        .arg("ship")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_confignoself_cliwithself_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]", "[defaults]", "ship.no-self = true"]);

    cargo_liner()
        .args(["ship", "--with-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_confignoself_envnoself_cliwithself_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]", "[defaults]", "ship.no-self = true"]);

    cargo_liner()
        .env("CARGO_LINER_SHIP_NO_SELF", "true")
        .args(["ship", "--with-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noupdate.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_confignoself_envnoself_clinoself_nothing() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]", "[defaults]", "ship.no-self = true"]);

    cargo_liner()
        .env("CARGO_LINER_SHIP_NO_SELF", "true")
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_configwithself_envnoself_clinoself_nothing() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]", "[defaults]", "ship.no-self = false"]);

    cargo_liner()
        .env("CARGO_LINER_SHIP_NO_SELF", "true")
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_configwithself_envwithself_clinoself_nothing() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]", "[defaults]", "ship.no-self = false"]);

    cargo_liner()
        .env("CARGO_LINER_SHIP_NO_SELF", "false")
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

#[cargo_test]
fn validate_ship_confignoself_envwithself_clinoself_noupdate() {
    let _reg = init_registry();
    fake_publish("cargo-liner", "0.0.3");
    fake_install("cargo-liner", "0.0.3", false);
    assert_installed("cargo-liner");
    write_user_config(&["[packages]", "[defaults]", "ship.no-self = true"]);

    cargo_liner()
        .env("CARGO_LINER_SHIP_NO_SELF", "false")
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file!["fixtures/ship/validate_ship_noself_nothing.stderr"].raw());
    assert_installed("cargo-liner");
}

/// Test `--dry-run` for the Install method.
#[cargo_test]
fn validate_ship_install_dryrun() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    write_user_config(&["[packages]", "abc = '*'"]);

    cargo_liner()
        .args(["ship", "--no-self", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_install_dryrun.stderr"
        ]);
    assert_not_installed("abc");
}

/// See #27.
#[cargo_test]
fn validate_ship_path_is_skipcheck() {
    let _reg = init_registry();
    fake_install("abc", "0.0.0", true);
    fake_publish("abc", "0.0.1");
    write_user_config(&["[packages]", "abc = { version = '*', path = '/a' }"]);

    // HACK: validate that the error reveals the intended behavior.
    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_path_is_skipcheck.stderr"
        ]);
    assert_installed("abc");
}

/// See #27.
#[cargo_test]
fn validate_ship_git_is_skipcheck() {
    let _reg = init_registry();
    fake_install("abc", "0.0.0", true);
    fake_publish("abc", "0.0.1");
    write_user_config(&[
        "[packages]",
        "abc = { version = '*', git = 'git@example.com:user/repo.git' }",
    ]);

    // HACK: validate that the error reveals the intended behavior.
    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_git_is_skipcheck.stderr"
        ]);
    assert_installed("abc");
}

/// Test that the tool is at least called when using `always` globally.
///
/// Binstall seems to outright refuse http URLs, so it is rather incompatible
/// with the execution context used here. Just at least validating the tool is
/// indeed called seems to be the only possibility here.
#[ignore = "cargo-binstall is not made available to CI yet."]
#[cargo_test]
fn validate_ship_binstall_globalalways_isused() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    write_user_config(&[
        "[defaults]",
        "ship.binstall = 'always'",
        "[packages]",
        // Explicitly passing the registry is necessary as otherwise crates.io
        // is implicitly used by the tool that does not have the same hacks.
        "abc = { version = '*', registry = 'dummy-registry' }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_binstall_globalalways_isused.stderr"
        ]);
    assert_not_installed("abc");
}

/// Test that the tool is not used when using `never` globally.
///
/// This is the same as if only letting `auto` since it is disabled under tests.
#[ignore = "cargo-binstall is not made available to CI yet."]
#[cargo_test]
fn validate_ship_binstall_globalnever_isunused() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    write_user_config(&[
        "[defaults]",
        "ship.binstall = 'never'",
        "[packages]",
        "abc = { version = '*', registry = 'dummy-registry' }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_binstall_globalnever_isunused.stderr"
        ]);
    assert_installed("abc");
}

/// Test that the per-package configuration has precedence over the global one.
#[ignore = "cargo-binstall is not made available to CI yet."]
#[cargo_test]
fn validate_ship_binstall_globalnever_localalways_isused() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    write_user_config(&[
        "[defaults]",
        "ship.binstall = 'never'",
        "[packages]",
        "abc = { version = '*', binstall = 'always', registry = 'dummy-registry' }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_binstall_globalalways_isused.stderr"
        ]);
    assert_not_installed("abc");
}

/// Test that the per-package configuration has precedence over the global one.
#[ignore = "cargo-binstall is not made available to CI yet."]
#[cargo_test]
fn validate_ship_binstall_globalalways_localnever_isunused() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    write_user_config(&[
        "[defaults]",
        "ship.binstall = 'always'",
        "[packages]",
        "abc = { version = '*', binstall = 'never', registry = 'dummy-registry' }",
    ]);

    cargo_liner()
        .args(["ship", "--no-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        // Same as above.
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_binstall_globalnever_isunused.stderr"
        ]);
    assert_installed("abc");
}

/// Test `--dry-run` for the Binstall method.
#[ignore = "cargo-binstall is not made available to CI yet."]
#[cargo_test]
fn validate_ship_binstall_dryrun() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    write_user_config(&[
        "[defaults]",
        "ship.binstall = 'always'",
        "[packages]",
        "abc = { version = '*', registry = 'dummy-registry' }",
    ]);

    // It fails here, but the logs show the option is passed.
    cargo_liner()
        .args(["ship", "--no-self", "--dry-run"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_binstall_dryrun.stderr"
        ]);
    assert_not_installed("abc");
}

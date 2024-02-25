use cargo_test_macro::cargo_test;
use cargo_test_support::registry::Package;
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_noupdate.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_noupdate.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_noself_nothing.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_noself_nothing.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_noupdate.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_noupdate.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_noself_nothing.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_noself_nothing.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_noself_nothing.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_noself_nothing.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
            "fixtures/ship/validate_ship_manynewer_update.stderr"
        ]);
    fixture_assert_installed();
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_eq("    Updating `dummy-registry` index\n");
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_skipcheck_noself_nothing.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
            "fixtures/ship/validate_ship_cargoinstallroot_supported.stderr"
        ]);
    assert!(tmp_dir.path().join("bin/pkg").is_file());
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .success()
        .stdout_eq("")
        .stderr_matches(snapbox::file![
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
        .stdout_eq("")
        .stderr_matches(snapbox::file![
            "fixtures/ship/validate_ship_features.stderr"
        ]);
    assert_installed("pkg");
}

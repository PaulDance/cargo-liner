use cargo_test_macro::cargo_test;
use cargo_test_support::registry::Package;
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
        .stderr_eq("    Updating `dummy-registry` index\n".raw());
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
    assert_not_installed("abc");
    assert_not_installed("def");
}

#[cargo_test]
fn validate_ship_install_keepgoing_err_iserr() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("def", "0.0.0");
    write_user_config(&["[packages]", "abc = '*'", "def = '*'"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self", "--keep-going"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_install_keepgoing_err_iserr.stderr"
        ]);
    assert_not_installed("abc");
    assert_installed("def");
}

#[cargo_test]
fn validate_ship_install_keepgoing_ok_isok() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish_all([("abc", "0.0.0"), ("def", "0.0.0")]);
    write_user_config(&["[packages]", "abc = '*'", "def = '*'"]);

    cargo_liner()
        .args(["ship", "--skip-check", "--no-self", "--keep-going"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/ship/validate_ship_install_keepgoing_ok_isok.stderr"
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

use cargo_test_macro::cargo_test;

mod common;
use common::*;

fn fixture_fake_install() {
    fake_install_all([("abc", "0.0.1"), ("def", "0.0.2"), ("cargo-liner", "0.0.3")]);
}

#[cargo_test]
fn validate_import() {
    fixture_fake_install();
    cargo_liner()
        .arg("import")
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_keepself() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--keep-self"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import_keepself.outconfig");
}

#[cargo_test]
fn validate_import_exact() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--exact"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import_exact.outconfig");
}

#[cargo_test]
fn validate_import_exact_keepself() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--exact", "--keep-self"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import_exact_keepself.outconfig");
}

#[cargo_test]
fn validate_import_compatible() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--compatible"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import_compatible.outconfig");
}

#[cargo_test]
fn validate_import_compatible_keepself() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--compatible", "--keep-self"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path(
        "tests/fixtures/import/validate_import_compatible_keepself.outconfig",
    );
}

#[cargo_test]
fn validate_import_patch() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--patch"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import_patch.outconfig");
}

#[cargo_test]
fn validate_import_patch_keepself() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--patch", "--keep-self"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import_patch_keepself.outconfig");
}

#[cargo_test]
fn validate_import_force_nofile_isok() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--force"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_force_withfile_iswarn() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["import", "--force"])
        .assert()
        .success()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import_force_warn.stdout");
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_noforce_withfile_iserr() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner()
        .arg("import")
        .assert()
        .failure()
        .stderr_eq("")
        .stdout_eq_path("tests/fixtures/import/validate_import_force_err.stdout");
    assert_user_config_eq("[packages]");
}
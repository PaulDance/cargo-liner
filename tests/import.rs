use cargo_test_macro::cargo_test;

mod common;
use common::*;

#[cargo_test]
fn validate_import() {
    fixture_fake_install();
    cargo_liner()
        .arg("import")
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_keepself() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import_keepself.outconfig");
}

#[cargo_test]
fn validate_import_keeplocal() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--keep-local"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import_keeplocal.outconfig");
}

#[cargo_test]
fn validate_import_keepself_keeplocal() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--keep-local", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path(
        "tests/fixtures/import/validate_import_keepself_keeplocal.outconfig",
    );
}

#[cargo_test]
fn validate_import_exact() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--exact"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import_exact.outconfig");
}

#[cargo_test]
fn validate_import_exact_keepself() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--exact", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import_exact_keepself.outconfig");
}

#[cargo_test]
fn validate_import_compatible() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--compatible"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import_compatible.outconfig");
}

#[cargo_test]
fn validate_import_compatible_keepself() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--compatible", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
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
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import_patch.outconfig");
}

#[cargo_test]
fn validate_import_patch_keepself() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--patch", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import_patch_keepself.outconfig");
}

#[cargo_test]
fn validate_import_force_nofile_isok() {
    fixture_fake_install();
    cargo_liner()
        .args(["import", "--force"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"]);
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_force_warn.stderr"
        ]);
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
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_force_err.stderr"
        ]);
    assert_user_config_eq("[packages]");
}

#[cargo_test]
fn validate_import_verbosity_v() {
    fixture_fake_install();

    cargo_liner()
        .args(["-v", "import"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_matches(snapbox::file![
            "fixtures/import/validate_import_verbosity_v.stderr"
        ]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_vv() {
    fixture_fake_install();

    cargo_liner()
        .args(["-vv", "import"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_matches(snapbox::file![
            "fixtures/import/validate_import_verbosity_v.stderr"
        ]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_vvv() {
    fixture_fake_install();

    cargo_liner()
        .args(["-vvv", "import"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_matches(snapbox::file![
            "fixtures/import/validate_import_verbosity_vvv.stderr"
        ]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_q_force() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["-q", "import", "--force"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_verbosity_q_force.stderr"
        ]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_qq_force() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["-qq", "import", "--force"])
        .assert()
        .success()
        .stdout_eq("")
        .stderr_eq("");
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_qq_noforce() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["-qq", "import"])
        .assert()
        .failure()
        .stdout_eq("")
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_force_err.stderr"
        ]);
    assert_user_config_eq("[packages]");
}

#[cargo_test]
fn validate_import_verbosity_qqq_noforce() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner()
        .args(["-qqq", "import"])
        .assert()
        .failure()
        .stdout_eq("")
        .stderr_eq("");
    assert_user_config_eq("[packages]");
}

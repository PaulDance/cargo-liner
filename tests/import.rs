use cargo_test_macro::cargo_test;
use snapbox::IntoData;

mod common;
use common::*;

#[cargo_test]
fn validate_import() {
    fixture_fake_install();
    cargo_liner!()
        .arg("import")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_keepself() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import_keepself.outconfig");
}

#[cargo_test]
fn validate_import_keeplocal() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--keep-local"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import_keeplocal.outconfig");
}

#[cargo_test]
fn validate_import_keepself_keeplocal() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--keep-local", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path(
        "tests/fixtures/import/validate_import_keepself_keeplocal.outconfig",
    );
}

#[cargo_test]
fn validate_import_exact() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--exact"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import_exact.outconfig");
}

#[cargo_test]
fn validate_import_exact_keepself() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--exact", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import_exact_keepself.outconfig");
}

#[cargo_test]
fn validate_import_compatible() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--compatible"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import_compatible.outconfig");
}

#[cargo_test]
fn validate_import_compatible_keepself() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--compatible", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path(
        "tests/fixtures/import/validate_import_compatible_keepself.outconfig",
    );
}

#[cargo_test]
fn validate_import_patch() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--patch"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import_patch.outconfig");
}

#[cargo_test]
fn validate_import_patch_keepself() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--patch", "--keep-self"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import_patch_keepself.outconfig");
}

#[cargo_test]
fn validate_import_force_nofile_isok() {
    fixture_fake_install();
    cargo_liner!()
        .args(["import", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_force_withfile_iswarn() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["import", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/import/validate_import_force_warn.stderr"].raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_noforce_withfile_iserr() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("import")
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_force_err.stderr"
        ]);
    assert_user_config_eq("[packages]");
}

#[cargo_test]
fn validate_import_nofile_iserr() {
    cargo_liner!()
        .arg("import")
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_nofile_iserr.stderr"
        ]);
    assert_user_config_absent();
}

#[cargo_test]
fn validate_import_verbosity_v() {
    fixture_fake_install();

    cargo_liner!()
        .args(["-v", "import"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_verbosity_v.stderr"
        ]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_vv() {
    fixture_fake_install();

    cargo_liner!()
        .args(["-vv", "import"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_verbosity_v.stderr"
        ]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_vvv() {
    fixture_fake_install();

    cargo_liner!()
        .args(["-vvv", "import"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_verbosity_vvv.stderr"
        ]);
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_v_nofile_iserr() {
    cargo_liner!()
        .args(["-v", "import"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_verbosity_v_nofile_iserr.stderr"
        ]);
    assert_user_config_absent();
}

#[cargo_test]
fn validate_import_verbosity_vv_nofile_iserr() {
    cargo_liner!()
        .args(["-vv", "import"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_verbosity_vv_nofile_iserr.stderr"
        ]);
    assert_user_config_absent();
}

#[cargo_test]
fn validate_import_verbosity_vvv_nofile_iserr() {
    cargo_liner!()
        .args(["-vvv", "import"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            // Intentionally the same as the test above: nothing should change.
            "fixtures/import/validate_import_verbosity_vv_nofile_iserr.stderr"
        ]);
    assert_user_config_absent();
}

#[cargo_test]
fn validate_import_verbosity_q_force() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["-q", "import", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(
            snapbox::file!["fixtures/import/validate_import_verbosity_q_force.stderr"].raw(),
        );
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_qq_force() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["-qq", "import", "--force"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq("".into_data().raw());
    assert_user_config_eq_path("tests/fixtures/import/validate_import.outconfig");
}

#[cargo_test]
fn validate_import_verbosity_qq_noforce() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["-qq", "import"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/import/validate_import_force_err.stderr"
        ]);
    assert_user_config_eq("[packages]");
}

#[cargo_test]
fn validate_import_verbosity_qqq_noforce() {
    fixture_fake_install();
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["-qqq", "import"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq("".into_data().raw());
    assert_user_config_eq("[packages]");
}

use cargo_test_macro::cargo_test;
use snapbox::IntoData;

mod common;
use common::*;

#[cargo_test]
fn validate_jettison_selfstays() {
    fake_install_self();
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("jettison")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file!["fixtures/jettison/validate_jettison_nothing.stderr"].raw());
    assert_installed_self();
}

#[cargo_test]
fn validate_jettison_uninstall_one() {
    fake_install_self();
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("jettison")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_uninstall_one.stderr"
        ]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_jettison_install_then_uninstall_one() {
    let _reg = init_registry();
    fake_install_self();
    fake_publish("abc", "0.0.0");
    assert_not_installed("abc");

    write_user_config(&["[packages]", "abc = '*'"]);
    cargo_liner!()
        .args(["ship", "--no-self"])
        .assert()
        .success();
    assert_installed("abc");

    write_user_config(&["[packages]"]);
    cargo_liner!()
        .arg("jettison")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_uninstall_one.stderr"
        ]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_jettison_uninstall_all() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("jettison")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_uninstall_multiple.stderr"
        ]);
    assert_not_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_uninstall_some() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
        ("jkl", "0.0.0", false),
        ("mno", "0.0.0", false),
        ("pqr", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi", "jkl", "mno", "pqr"]);
    write_user_config(&["[packages]", "jkl = '*'", "mno = '*'", "pqr = '*'"]);

    cargo_liner!()
        .arg("jettison")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_uninstall_multiple.stderr"
        ]);
    assert_not_installed_all(["abc", "def", "ghi"]);
    assert_installed_all(["jkl", "mno", "pqr"]);
}

#[cargo_test]
fn validate_jettison_verbosity_and_color() {
    fake_install_self();
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]"]);

    // HACK: use the debug output to observe their being passed down.
    cargo_liner!()
        .args(["jettison", "-vvv", "--color=never"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_verbosity_and_color.stderr"
        ]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_jettison_confirm_continue_y() {
    fake_install_self();
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("jettison")
        .stdin("y")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_confirm_continue.stderr"
        ]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_jettison_confirm_continue_lf() {
    fake_install_self();
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("jettison")
        .stdin("")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_confirm_continue.stderr"
        ]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_jettison_confirm_abort() {
    fake_install_self();
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("jettison")
        .stdin("n")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_confirm_abort.stderr"
        ]);
    assert_installed("abc");
}

#[cargo_test]
fn validate_jettison_confirm_retry_continue() {
    fake_install_self();
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("jettison")
        .stdin("a\nb\nc\ny")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_confirm_retry_continue.stderr"
        ]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_jettison_confirm_retry_abort() {
    fake_install_self();
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .arg("jettison")
        .stdin("a\nb\nc\nn")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_confirm_retry_abort.stderr"
        ]);
    assert_installed("abc");
}

#[cargo_test]
fn validate_jettison_noconfirm() {
    fake_install_self();
    fake_install("abc", "0.0.0", false);
    assert_installed("abc");
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["jettison", "--no-confirm"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_noconfirm.stderr"
        ]);
    assert_not_installed("abc");
}

#[cargo_test]
fn validate_jettison_dryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["jettison", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_failfast() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]"]);
    break_fake_installation("def");

    cargo_liner!()
        .arg("jettison")
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_failfast.stderr"
        ]);
    // The assertion checks for the exe only, so counts as uninstalled here.
    assert_not_installed_all(["abc", "def"]);
    assert_installed("ghi");
}

#[cargo_test]
fn validate_jettison_nofailfast_err_iserr() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
        ("jkl", "0.0.0", false),
        ("mno", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]"]);
    break_fake_installation_all(["def", "jkl"]);

    cargo_liner!()
        .args(["jettison", "--no-fail-fast"])
        .assert()
        .failure()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_nofailfast_err_iserr.stderr"
        ]);
    assert_not_installed_all(["abc", "def", "ghi", "jkl", "mno"]);
}

#[cargo_test]
fn validate_jettison_nofailfast_ok_isok() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["jettison", "--no-fail-fast"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_uninstall_multiple.stderr"
        ]);
    assert_not_installed_all(["abc", "def", "ghi"]);
}

// Precedence testing.

#[cargo_test]
fn validate_jettison_configdryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = true"]);

    cargo_liner!()
        .arg("jettison")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_envdryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "true")
        .arg("jettison")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_clidryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .args(["jettison", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_confignodryrun_envdryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = false"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "true")
        .arg("jettison")
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_confignodryrun_clidryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = false"]);

    cargo_liner!()
        .args(["jettison", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_envnodryrun_clidryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "false")
        .args(["jettison", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_confignodryrun_envnodryrun_clidryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = false"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "false")
        .args(["jettison", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_configdryrun_envnodryrun_clidryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = true"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "false")
        .args(["jettison", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_configdryrun_envdryrun_clidryrun_isdryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = true"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "true")
        .args(["jettison", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_configdryrun_envdryrun_clinodryrun_isnodryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = true"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "true")
        .args(["jettison", "--no-dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_uninstall_multiple.stderr"
        ]);
    assert_not_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_confignodryrun_envdryrun_clinodryrun_isnodryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = false"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "true")
        .args(["jettison", "--no-dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_uninstall_multiple.stderr"
        ]);
    assert_not_installed_all(["abc", "def", "ghi"]);
}

#[cargo_test]
fn validate_jettison_confignodryrun_envnodryrun_clinodryrun_isnodryrun() {
    fake_install_self();
    fake_install_all([
        ("abc", "0.0.0", false),
        ("def", "0.0.0", false),
        ("ghi", "0.0.0", false),
    ]);
    assert_installed_all(["abc", "def", "ghi"]);
    write_user_config(&["[packages]", "[defaults.jettison]", "dry-run = false"]);

    cargo_liner!()
        .env("CARGO_LINER_JETTISON_DRY_RUN", "false")
        .args(["jettison", "--no-dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_uninstall_multiple.stderr"
        ]);
    assert_not_installed_all(["abc", "def", "ghi"]);
}

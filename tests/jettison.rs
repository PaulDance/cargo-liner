use cargo_test_macro::cargo_test;
use snapbox::IntoData;

mod common;
use common::*;

#[cargo_test]
fn validate_jettison_selfstays() {
    fake_install_self();
    write_user_config(&["[packages]"]);

    cargo_liner()
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

    cargo_liner()
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
    cargo_liner().args(["ship", "--no-self"]).assert().success();
    assert_installed("abc");

    write_user_config(&["[packages]"]);
    cargo_liner()
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

    cargo_liner()
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

    cargo_liner()
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
    cargo_liner()
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

    cargo_liner()
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

    cargo_liner()
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

    cargo_liner()
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

    cargo_liner()
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

    cargo_liner()
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

    cargo_liner()
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

    cargo_liner()
        .args(["jettison", "--dry-run"])
        .assert()
        .success()
        .stdout_eq("".into_data().raw())
        .stderr_eq(snapbox::file![
            "fixtures/jettison/validate_jettison_dryrun.stderr"
        ]);
    assert_installed_all(["abc", "def", "ghi"]);
}

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

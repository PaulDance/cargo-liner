use std::process;

use cargo_test_macro::cargo_test;
use snapbox::cmd::Command;
use snapbox::data::DataFormat;
use snapbox::{Data, IntoData};

mod common;
use common::*;

fn validate_shell(shell: &str, run: bool) {
    cargo_liner()
        .args(["completions", shell])
        .assert()
        .success()
        .stderr_eq_("...")
        .stdout_eq_(
            Data::read_from(
                &snapbox::utils::current_dir!()
                    .join(format!("fixtures/completions/validate_{shell}.stdout")),
                Some(DataFormat::Binary),
            )
            .raw(),
        );

    // Actually run the script to see what happens, but only if asked to and
    // the given shell is available from the current environment.
    if run {
        if let Ok(status) = process::Command::new(shell).arg("--version").status() {
            if status.success() {
                Command::new(shell)
                    .arg(format!(
                        "tests/fixtures/completions/validate_{shell}.stdout"
                    ))
                    .assert()
                    .success()
                    .stdout_eq_("".into_data().raw())
                    .stderr_eq_("".into_data().raw());
            }
        }
    }
}

#[cargo_test]
fn validate_bash() {
    validate_shell("bash", true);
}

#[cargo_test]
fn validate_zsh() {
    // Running the script with Zsh does not work... while it works.
    validate_shell("zsh", false);
}

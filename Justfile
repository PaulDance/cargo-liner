set fallback := true

# Lists available recipes.
default:
    @just --list

alias c := check
# Runs `cargo check` with some default flags.
check *args:
    cargo check --all-targets --all-features {{ args }}

alias b := build
# Runs `cargo build` with some default flags.
build *args:
    cargo build --all-targets --all-features {{ args }}

alias r := run
# Runs `cargo run` with some default flags.
run *args:
    cargo run --all-features {{ args }}

alias t := test
# Runs `cargo test` with some default flags.
test *args:
    cargo test --all-targets --all-features {{ args }}

alias cl := clippy
# Runs `cargo clippy` with some default flags.
clippy *args:
    cargo clippy --all-targets --all-features {{ args }}

alias d := doc
# Runs `cargo doc` with some default flags.
doc *args:
    cargo doc --all-features --document-private-items {{ args }}

alias f := fmt
# Alias for `fmt --check`.
fc *args: (fmt "--check" args)
# Runs `cargo +nightly fmt` with some default flags.
fmt *args:
    cargo +nightly fmt --all {{ args }}

alias ca := check-all
# Runs all checks in sequence.
check-all: check build test clippy doc fc

alias ur := update-readme
# Instructs Trycmd to overwrite any out-of-date `console` code blocks in the `README.md`.
update-readme:
    TRYCMD=overwrite cargo test --test readme -- validate_readme

alias uf := update-fixtures
# Instructs Snapbox to overwrite any out-of-date test fixture files.
update-fixtures *args:
    SNAPSHOTS=overwrite cargo test --all-features --tests -- --exact --skip validate_readme {{ args }}

alias uc := update-cargo
# Runs `cargo update` with some default flags.
update-cargo *args:
    cargo update --recursive {{ args }}

alias p := publish
# Runs a Cargo update, all checks and `cargo publish`.
[confirm]
publish: update-cargo check-all
    cargo publish

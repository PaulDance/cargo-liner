# Unreleased
## Added
 * Package definitions can now specify which feature flags should be installed,
   in a similar format to the Cargo.toml `[features]` section.

# [Version 0.2.1 (26/11/2022)](https://crates.io/crates/cargo-liner/0.2.1)
## Fixes

 * The `import` sub-command now filters this project's package name out of the
   packages when importing. Avoids fixing to a version needlessly or adding a
   useless line considering the already-implemented self-update feature. Tests
   now also check for this bug.
 * Added the missing `wrap_help` feature to Clap in order to have prettier help
   and usage messages: they now adapt themselves to the current terminal size.

## Documentation

 * `README.md`:
   * Fixed import suggestion in installation steps.
   * Made a proper Summary section.
   * Fixed a small punctuation mistake.
   * Added an example output for the `ship` sub-command.
   * Added appropriate links to badges.
 * `CHANGELOG.md`: added links to crates.io versions.
 * Repository: added issue templates.


# [Version 0.2.0 (20/11/2022)](https://crates.io/crates/cargo-liner/0.2.0)
## Features

 * Self-updating enabled by default.
 * New `ship` subcommand to materialize the default behavior.
   * New `--no-self-update` option to disable the by-default self-updating.
 * New `import` subcommand to convert `$CARGO_HOME/.crates.toml` to
   `$CARGO_HOME/liner.toml`.
   * New `--force` option to overwrite the destination file.
   * New `--exact`, `--patch`, and `--compatible` options to customize imported
     version requirements.

## Testing

 * New tests for the new features.
 * Some basic refactoring.

## Documentation

 * Some minor spelling mistakes have been fixed in `README.md`.
 * CI results badge has been added to `README.md`.
 * `CHANGELOG.md`'s format has been adjusted slightly.
 * Documentation for new features.


# [Version 0.1.1 (16/11/2022)](https://crates.io/crates/cargo-liner/0.1.1)
## Fixes

 * The `deny_unknown_fields` serde attribute on the config parser has been
   removed: it ensures forward and backward compatibility.

## Testing

 * New basic tests have been implemented for some modules.
 * GitHub Actions have been set up in order to provide CI.

## Documentation

 * New files: `CONTRIBUTING.md` and `CODE_OF_CONDUCT.md`.
 * Sexy badges have been added to the `README.md`.
 * The `CHANGELOG.md` has been tweaked a bit to include version dates.


# [Version 0.1.0 (13/11/2022)](https://crates.io/crates/cargo-liner/0.1.0)
## Features

 * Most basic CLI.
 * Deserialization and validation of configuration file.
 * Call to `cargo install` with configured package names and versions.

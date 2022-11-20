# Version 0.2.0 (20/11/2022)
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


# Version 0.1.1 (16/11/2022)
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


# Version 0.1.0 (13/11/2022)
## Features

 * Most basic CLI.
 * Deserialization and validation of configuration file.
 * Call to `cargo install` with configured package names and versions.

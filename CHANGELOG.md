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

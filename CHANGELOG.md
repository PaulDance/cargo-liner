<!-- Version notes template:
# [Version 0.. (//)](https://crates.io/crates/cargo-liner/0..)
## Features
## Fixes
## Testing
## Documentation
## Miscellaneous
-->

# [Version 0.7.0 (26/05/2024)](https://crates.io/crates/cargo-liner/0.7.0)
## Features

 * Completed #24: a new `completions` subcommand has been introduced in order
   to enable one to generate shell CLI auto-completion scripts for the program
   this project delivers. More could be added to this in the future, but for
   now, the simplest is to add it to one's shell configuration to be loaded at
   each startup so it may be dynamically up-to-date.

 * Completed #11: a new `--keep-going` CLI option has been added to the `ship`
   subcommand. It enables one to bypass the new by-default fail-fast behavior
   of the command in order to make it ignore `cargo install` errors instead and
   continue on calling it for other packages. This can be used as a way to
   still install or update the remaining packages when one fails, but not
   without having read the error message first and decided what was the best
   course for remediation beforehand.

 * Completed #23's first part: a new `extra-arguments` package option has been
   added to the user configuration file. It enables one to specify additional
   CLI arguments that should be given to the call to `cargo install` associated
   with the concerned package. They are not used for other packages.

 * Completed #23's second part: a new `environment` package option has been
   added to the user configuration file. It enables one to specify environment
   variable names and values that should be set for the `cargo install` process
   spawned for the associated package. They are not used for other packages.

## Fixes

 * Part of #11: previously, whenever some package would fail to install for any
   reason reported by Cargo, the overall operation would always ignore the
   error as it was simply not checked and continue on with the rest of the
   packages, which somewhat worked, but wasn't the clearest as the message
   could simply slip by unnoticed among the rest of the numerous Cargo output
   lines. Now, the exit status of the `cargo install` process is properly
   checked and when unsuccessful, makes the overall operation stop as soon as
   possible, thus making it fail-fast by default.

## Testing

 * The fixtures have been updated to take changes to Cargo, dependencies and
   internal refactors into account.
 * Added coverage for the new fixes and features.

## Documentation

 * `README.md`:
   * Fixed a link to cargo install docs.
   * Added a clarification about what the `import` subcommand ignores by
     default and that was missing in the previous release.
   * Added the missing documentation about the various configuration sections,
     expected and optional, and how and what they are used for.
   * Documented the new features.
 * `CHANGELOG.md`: new version.

## Miscellaneous

 * The dependencies have been updated.
 * Recent changes to Cargo and Clippy have been taken into account.
 * More lints have been enabled on the codebase.
 * Added a suggestion message to use the `import` subcommand when `ship` fails
   because of the `liner.toml` configuration file's not existing.
 * The global error message displayed when an installation fails has been
   adapted to take into account whether `--keep-going` has been used or not so
   it may be a bit clearer and explicit.
 * Added a suggestion message to the error message displayed when `ship` is
   called and `cargo install` fails, to use the `--keep-going` option when not
   set. This should guide one to use the new feature when appropriate.


# [Version 0.6.0 (26/02/2024)](https://crates.io/crates/cargo-liner/0.6.0)
## Features

 * Merged #19 that closed #13: the version report displayed early in the
   execution of the `ship` subcommand now uses [`tabled`] in order to present
   it in the form of a table, which looks much better while only requiring
   little code to achieve it. An example of it can be found in the updated
   `README.md`. Many thanks to @ToBinio.

 * Merged #22 that closed #21: the `import` subcommand now ignores by default
   all locally-installed packages, i.e. the ones installed through `cargo
   install --path=...`. This should help to avoid poluting the destination
   configuration file with packages that cannot be updated anyway since they
   have no guarantee of existing in the registry used. If the previous behavior
   is still desired however, a new `--keep-local` CLI option has been added for
   this reason. As a side-effect, `--keep-self`'s shorthand `-k` has been
   changed to `-s` in order to make room for the new option's `-l`. Many thanks
   to @ToBinio.

 * The error reporting has been completely overhauled in order to add context
   through the call stacks. Thanks to [`color-eyre`], the errors now transport
   additional information indicating what exactly failed, why it could have
   happened so and how it could be fixed. This should help users diagnose and
   fix the usual issues quicker, such as missing files. It should also combine
   well with the CLI verbosity controls in case more precise debugging is still
   required as they now also control the verbosity of these error reports: when
   `-v` is given, the `RUST_BACKTRACE=1` format is used and when `-vv` or more
   is given, `RUST_BACKTRACE=full` is used; when `-qqq` or more is given, the
   error report is entirely skipped. It is also not displayed in the logs
   anymore, since the whole point is to have a proper termination type.

[`tabled`]: https://docs.rs/tabled/latest/tabled/
[`color-eyre`]: https://docs.rs/color-eyre/latest/color_eyre/

## Fixes

 * Merged #20: the build was broken for Windows because of some Unix-specific
   API that slipped in; it is now fixed. GNU/Linux is still the only platform
   tested in CI, but the others should work fine. Thanks to @ToBinio.

 * Made sure the exit status code of the `cargo search` child processes spawned
   in order to fetch the latest versions of the configured packages was checked
   before continuing. Previously, only errors specifically related to spawning
   and waiting for them were checked and bubbled up, but no manual check was
   actually done on the status code. The reported error should now be clearer.

## Testing

The fixtures have been updated and new tests have been added for:
 * The change of the version report.
 * The new `import` filtering feature and its `--keep-local` option.
 * The new error reporting.

## Documentation

 * `CONTRIBUTING.md`:
   * Added a mention for GPG signing as a requirement.
   * Added precisions about the development environment used.
   * Added a mention for the new error contexts.
 * `README.md`:
   * Updated the CLI help usage messages for the new features: the new import
     option and the changed error reporting.
   * Fixed the link to Cargo's `install` documentation.
   * Wrapped the example output lines to 69 columns at most. It was previously
     done at 70, but was apparently still too much by one for crates.io.
     Hopefully, it is now correct.
   * Updated the crates.io download stats badge's link to point at the bottom
     of the page in order to provide quicker access. Hopefully, it will work.
 * `CHANGELOG.md`:
   * Fixed a random typo lost somewhere.
   * Made a hidden version notes template for quicker releases.
   * Added a new version.

## Miscellaneous

 * The dependencies have been updated, which required updating some testing
   code to adapt to the changes included in Snapbox.
 * Introduced a stricter Rustfmt configuration for more control over the
   automatic formatting and to enforce things done manually.
 * Introduced a Justfile in order to provide some command shortcuts useful
   during development.
 * The MSRV has been bumped to 1.76 in order to use `Result::inspect_err`.


# [Version 0.5.0 (03/02/2024)](https://crates.io/crates/cargo-liner/0.5.0)
## Features

 * Added a new global CLI option: `--color`. As its name indicates, it controls
   the output coloring of logs and calls to Cargo. It takes precedence over any
   kind of environment variables.

 * When using one `--verbose` or more, the `ship` command now allows calls to
   the internal `cargo search` to inherit its standard error stream. This helps
   in debugging. During normal operation, it displays messages related to index
   and lock files management.

 * The verbosity controls now also control Cargo's own verbosity:
   * When `--verbose` is used twice, then `-v` is passed to it.
   * When `--verbose` is used three times or more, then `-vv` is passed to it.
   * When `--quiet` is used three times or more, then `-q` is passed to it.

## Fixes

 * Made sure the CLI verbosity controls had precedence over the environment:
   its variables were previously parsed after the CLI options were applied on
   the logging configuration, which was incorrect; the two have therefore been
   switched in order to fix the issue.

## Testing

 * Implemented integration tests to ensure the output of `--help` usage
   messages and other example outputs in the `README.md` were correct and could
   be automatically updated, thanks to `trycmd`.

 * Completed #10: using `snapbox` and Cargo's own testing framework, many
   integration tests have been implemented to cover the various features this
   project offers:
   * The fact that `cargo search` is used in order to check if new versions are
     available and that it works. Previously, these tests were disabled in CI
     because the external connections were blocked. Thanks to the new testing
     framework however, they were refactored to be offline and therefore set to
     run again in CI. They were also made single-threaded in order to avoid
     some observed flakiness since the framework does not seem to fit unit
     tests the best.
   * The `import` subcommand: ensure all combinations of CLI options produce
     correct and expected configurations.
   * The `ship` subcommand:
     * Test CLI options combinations.
     * Test install, update and no action.
     * Test that the feature control of the configuration is correctly passed
       onto the installation process.
   * The CLI verbosity control: ensure log messages and Cargo verbosity are
     controlled correctly.
   * The bugs previously fixed: regression tests.

## Documentation

 * Updated `README.md` multiple times when the above integration tests were
   added. This included wrapping to 70 columns instead of 80 in order to make
   the blocks present better on crates.io.
 * Removed a useless newline in the code of conduct.
 * Updated the contributing guidelines in order to mention the new tests.
 * Documented the new features in the CLI help messages.
 * The precedence of the verbosity CLI options over the environment has been
   clarified.

## Miscellaneous

 * The dependencies have been updated.
 * The new Git dev-dependencies introduce Cargo warnings that cannot be
   disabled. However, it is purely restricted to local usages and will not
   affect normal installations whatsoever. The issue is to be fixed in Cargo.
 * The `actions/checkout` GitHub Action has been updated.


# [Version 0.4.3 (17/01/2024)](https://crates.io/crates/cargo-liner/0.4.3)
## Fixes

 * Merged #14: previously, if the `CARGO_TERM_COLOR` environment variable was
   set to `always`, then the calls to `cargo search` and `cargo config` would
   fail as their respective output contained unexpected control sequences; the
   fix was therefore to always disable colors for every such call by adding the
   `--color=never` argument to each invoked Cargo process. `cargo install` was
   left untouched in this regard, however. Thanks to @pavel-procopiuc.

## Miscellaneous

 * The dependencies have been updated.
 * The MSRV has been downgraded to `1.70.0`. This is effectively a revert of
   the previous version's change. The bump has been reported in #14 to be too
   restrictive and the previous MSRV to work just fine.


# [Version 0.4.2 (05/01/2024)](https://crates.io/crates/cargo-liner/0.4.2)
## Fixes

 * Fixed #7: previously, if a configured package name started with a `-`, then
   the underlying call to Cargo would fail as expected since there is no
   package starting with such a character, however not on a search failure, but
   on a CLI parsing error: the name would be interpreted as a CLI option
   because there was no sanitization of package names. In order to fix it,
   rather than to implement actual parameter sanitization or validation, the
   missing `--` CLI option-argument separator was added to corresponding calls
   to `cargo install`. This way, any package name may be used and does not
   change the overall behavior. The registry is the one that does the
   validation, which is the most desirable.

 * Fixed #6: before, the version parsing logic used to extract the latest
   version from the results of the call to `cargo search` would be too strict
   and reject potential metadata suffixes, such as `v0.9.62-a.2` or
   `v13.0.0-alpha.0`. In order to fix this, the initial extraction was relaxed
   to accept a larger set of strings, but still piped into
   `semver::Version::parse`, which means the overall parsing is still strict,
   but more correct.

 * Merged #9: previously, the global process would always exit on a success
   status code, whether an error was encountered or not, which meant users
   could not detect such cases in an automated environment. This was fixed by
   returning [the standard failure exit code] of the current platform in case
   an error is encountered and a success otherwise. Thanks to @Johnabell.

[the standard failure exit code]: https://doc.rust-lang.org/stable/std/process/struct.ExitCode.html#associatedconstant.FAILURE

## Miscellaneous

 * The error messages related to a `cargo search` have been extended a bit in
   order to give a little bit more help to the user about the potential source
   of the true underlying issue: does the package actually exist?
 * The dependencies have been updated.
 * The MSRV has been bumped to `1.75.0`.


# [Version 0.4.1 (21/09/2023)](https://crates.io/crates/cargo-liner/0.4.1)
## Fixes

 * Fixed #5: before, the Cargo `.crates.toml` file would be read even if
   `--skip-check` was specified although it only used its contents in order to
   adjust a small part of the logging display that was somewhat useless; the
   option now also ensures reading from this file is avoided as well, which
   makes it even more able to work around some potential future bugs.

 * Fixed #4: the `$CARGO_INSTALL_ROOT` configuration possibility was previously
   not supported by the `.crates.toml`-reading operations, making the tool thus
   completely fail whenever used in such an environment; it now supports it
   entirely by wrapping calls to `cargo config get` in order to retrieve its
   value from either the environment or the `install.root` configuration key of
   the `$CARGO_HOME/config.toml` file, and falling back to the default
   `$CARGO_HOME/.crates.toml` if that fails for any reason, the simple absence
   of the setting being one of them. Whenever the first fails, it is logged as
   a `DEBUG` message before attempting the default, so use `-vv` to investiguate
   if your configuration seems not to be taken into account.

## Documentation

 * `README.md`:
   * Updated the verbatim help message of the `ship` command.
   * Simplified the description of the default command's operation to be a bit
     more summarized and then redirect to the more in-depth description found
     in the `ship` command's specific documentation section.
   * Mention the new `$CARGO_INSTALL_ROOT` support in various places.
   * Reformat the configuration examples to wrap long lines: better readability.
   * Some small rewordings.

## Miscellaneous

 * The dependencies have been updated.
 * Logging now uses pretty debug formatting, which splits arrays and structures
   onto multiple lines, therefore making reading long messages more comfortable.
 * The MSRV has been bumped to `1.70.0`.


# [Version 0.4.0 (28/06/2023)](https://crates.io/crates/cargo-liner/0.4.0)
## Features

 * The log messages will now display whether a package is already installed
   upon running the `ship` command when it is configured: either `Installing`
   when it was not previously installed or `Updating` on the contrary will be
   mentionned in the messages.

 * The `ship` command now uses `cargo search` in order to fetch the latest
   available version for each configured package before calling `cargo install`
   only for each of them that do indeed need an install or update when either
   not installed or installed with a version strictly older than the latest.
   The calls to `cargo search` are done in parallel, making this version check
   quite fast, therefore saving a lot of time by avoiding calling `cargo
   install` sequentially for each already-up-to-date package.

 * The `ship` command now displays a summarized update plan before running the
   actually necessary calls to `cargo install`. That makes the overall
   operation a bit more transparent and readable.

 * A new `--skip-check` option flag has been added to the `ship` command. As
   its name suggests, it enables one to skip the version check entirely. That
   means it will restore the previous simple behavior: run `cargo install` for
   each configured package, whether it needs an install or update or not. Most
   of the time, when updating already-installed packages for example, it will
   not prove very useful. However, it can be a bit quicker to use it when only
   very few packages are configured or if all or almost all are not alredy
   installed. It overall enables one to have a bit more control over the global
   operation of the tool, in case of an unexpected bug for example.

 * The new `--force` option flag has been implemented for the `ship` command.
   It simply passes it onto each call to `cargo install`: see its documentation
   for more information about it. When used in conjunction with `--skip-check`
   for example, it will redownload, recompile and reinstall each of the
   configured packages: use it only when truly necessary.

## Documentation

 * `README.md`:
   * Updated the example output to reflect the changes from the previous
     release but from this one as well.
   * Removed some useless trailing spaces.
   * New features.

## Testing

 * New tests have been added for the new CLI features, but also for the new
   functions wrapping calls to `cargo search`.

 * The GitHub Actions CI is now configured to skip the tests for these new
   search functions: the corresponding network calls seem to be filtered
   somehow.


# [Version 0.3.0 (19/06/2023)](https://crates.io/crates/cargo-liner/0.3.0)
## Features

 * Package definitions now have a detailed variant available, mimicking the
   `Cargo.toml` format. This has been implemented in order to enable one to
   specify which features should be compiled in the installed packages, by
   configuring a list of feature flags in a similar format to the Cargo
   manifest's `[features]` section. Many thanks to @MaeIsBad: #2.

 * A new `--only-self` option has been added to the `ship` command: it enables
   one to only update `cargo-liner` and nothing else. It is incompatible with
   `--no-self`.

 * Short variants of the options flags of the `ship` command are now enabled.

 * A new `--keep-self` option has been added to the `import` command: it
   ensures the `cargo-liner` package is kept in the package list and thus
   written to the created configuration file. It is compatible with the version
   operator selection options.

 * Basic logging has been implemented. Messages will therefore now be displayed
   in order to inform the user about the currently performed action and its
   results. Errors bubbling up are manually caught in the global main function
   and logged using the same format as the rest of the emitted messages.

 * New `--verbose` and `--quiet` option flags have been implemented. They
   control the global verbosity of the program by changing the logging
   configuration at runtime. They are both available globally, i.e. with or
   without a command and before or after the command name. They can both be
   repeated up to three times in order to adjust how much the verbosity is
   changed. They are incompatible with each other. It is made sure they arrive
   last in the CLI help messages, but still just before `--help` and
   `--version`.

## Fixes

 * The `--no-self-update` option of the `ship` command has been renamed to
   `--no-self`. This makes it more consistent with other new option flags.

 * The TOCTOU bug for the configuration file import has been fixed: between
   checking for the existence of the file and then possibly overwriting it, a
   new file could appear. The `--force` option flag now controls which
   operation is used: when present, the previously always used overwriting is
   called; when absent, an appropriate system call is made to ensure only
   creating a new file is possible atomically.

 * Fixed some Clippy warnings triggered on the test suite.

## Documentation

 * `README.md`:
   * Fixed the CI status badge.
   * Added missing shell syntax highlighting for the code block showing the
     output example.
   * Clarified CLI option defaults.
   * Updated outputs.
   * Various other small adjustments.
 * New features.

## Testing

 * The GitHub CI workflow now denies `rustdoc` warnings by default. Thanks
   to @MaeIsBad: #3.

 * Appropriate tests have been added alongside the new functionalities.


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

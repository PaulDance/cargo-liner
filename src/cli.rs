//! Module handling all the CLI arguments configuration and parsing.
//!
//! See [`LinerArgs::parse_env`] in order to retrieve such arguments from the
//! environment.
#![allow(clippy::struct_excessive_bools)]
use clap::{ArgAction, ColorChoice, Parser};
use clap_complete::Shell;

/// Cargo entry point for `cargo-liner`.
///
/// This tool is meant to be called using `cargo liner`.
#[derive(clap::Parser, Debug, PartialEq, Eq)]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoArgs {
    // The only variant: enables validating the input given by Cargo.
    Liner(LinerArgs),
}

impl CargoArgs {
    /// Unwraps the contained [`LinerArgs`] in an infallible way.
    pub fn liner(self) -> LinerArgs {
        match self {
            Self::Liner(args) => args,
        }
    }
}

// The actual entry point in this tool's argument parser.
#[derive(clap::Parser, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about)]
pub struct LinerArgs {
    #[command(subcommand)]
    pub command: Option<LinerCommands>,

    /// Be more verbose. Use multiple times to be more and more so each time.
    ///
    /// When omitted, INFO and above messages of only this crate are logged.
    /// When used once, DEBUG and above messages of only this crate are logged
    /// and error backtraces are shown (`RUST_BACKTRACE=1`).
    /// When used twice, DEBUG and above messages of all crates are logged,
    /// `-v` is given to Cargo calls (details ran commands), and error
    /// backtraces are fully shown (`RUST_BACKTRACE=full`).
    /// When used three times or more, TRACE and above messages of all crates
    /// are logged, `-vv` is given to Cargo calls (includes build output) and
    /// error backtraces are fully shown (`RUST_BACKTRACE=full`).
    /// This takes precedence over the environment.
    #[arg(
        short,
        long,
        global = true,
        action = ArgAction::Count,
        conflicts_with = "quiet",
        display_order = 989,
    )]
    pub verbose: u8,

    /// Be quieter. Use multiple times to be more and more so each time.
    ///
    /// When omitted, INFO and above messages of only this crate are logged.
    /// When used once, WARN and above messages of only this crate are logged.
    /// When used twice, ERROR messages of all crates are logged.
    /// When used three times or more, no message will be logged, including
    /// Cargo's by passing `-q` to it and error reports are silenced.
    /// This takes precedence over the environment.
    #[arg(
        short,
        long,
        global = true,
        action = ArgAction::Count,
        conflicts_with = "verbose",
        display_order = 990,
    )]
    pub quiet: u8,

    /// Control the coloring of the logging output.
    ///
    /// This enables one to manually specify when should the logs and error
    /// reports be colored or not, for example if the automatic detection is
    /// either not wished or not functional. The value is also passed onto
    /// calls to Cargo.
    #[arg(
        long,
        global = true,
        required = false,
        default_value = "auto",
        value_name = "WHEN",
        display_order = 991
    )]
    pub color: ColorChoice,
}

impl LinerArgs {
    /// Parses the arguments from the environment and returns them.
    ///
    /// Although it does not return `eyre::Result<Self>`, the function is
    /// actually fallible: it will print an error to stderr and exit the current
    /// process on an error status code if a parsing error occurs.
    pub fn parse_env() -> Self {
        CargoArgs::parse().liner()
    }

    /// Returns the requested verbosity level as a number.
    ///
    ///  * Zero means no option was provided: use the default.
    ///  * Positive numbers mean increased verbosity: `-vvv...`.
    ///  * Negative numbers mean decreased verbosity: `-qqq...`.
    pub fn verbosity(&self) -> i8 {
        if self.verbose > 0 {
            Self::u8_to_i8_saturating(self.verbose)
        } else {
            -Self::u8_to_i8_saturating(self.quiet)
        }
    }

    /// Converts the given `u8` to an `i8` by capping it to the max first.
    fn u8_to_i8_saturating(x: u8) -> i8 {
        x.min(i8::MAX as _).try_into().unwrap()
    }
}

/// Subcommands for the main CLI.
#[derive(clap::Subcommand, Debug, PartialEq, Eq)]
pub enum LinerCommands {
    /// The default command if omitted: install and update configured packages.
    ///
    /// Self-updating is enabled by default.
    Ship(ShipArgs),

    /// Import the `$CARGO_HOME/.crates.toml` Cargo-edited save file as a new
    /// Liner configuration file.
    ///
    /// Star versions are used by default. The version transformation options
    /// are mutually exclusive.
    Import(ImportArgs),

    /// Generate an auto-completion script for the given shell.
    ///
    /// The script is generated for `cargo-liner`, but with arguments rooted on
    /// `cargo-liner liner`, thus making auto-completing work when typing
    /// `cargo liner`. The generated script is emitted to standard output.
    Completions(CompletionsArgs),
}

/// Arguments for the `ship` subcommand.
#[derive(clap::Args, Debug, PartialEq, Eq)]
#[command(long_about = None)]
pub struct ShipArgs {
    /// Disable self-updating.
    ///
    /// Cannot be used in conjunction with `--only-self`. Default: `false`,
    /// i.e. self-update.
    #[arg(short, long, conflicts_with("only_self"))]
    pub no_self: bool,

    /// Only self-update and do not install or update any other package.
    ///
    /// Cannot be used in conjunction with `--no-self`. Default: `false`, i.e.
    /// install or update other packages as well.
    #[arg(short = 's', long, conflicts_with("no_self"))]
    pub only_self: bool,

    /// Skip the summary version check and directly call `cargo install` on
    /// each configured package.
    ///
    /// The version check is relatively quick and enables skipping calls to
    /// `cargo install` when no update is required, which saves quite a bit of
    /// time. However, if you wish, this option is still available in order not
    /// to run the check: doing so will probably take more time in the end most
    /// of the time, except if you have a very small amount of packages
    /// configured (e.g. one or two) or if all or almost all packages are not
    /// already installed.
    ///
    /// It can also be used as a workaround in case a certain operation fails
    /// in your particular environment, for example: reading from `.crates.toml`
    /// under the `$CARGO_HOME` or `$CARGO_INSTALL_ROOT` directory or making
    /// requests to the registry. These operations will thus be entirely
    /// skipped.
    #[arg(short = 'c', long)]
    pub skip_check: bool,

    /// Disable the default fail-fast execution of `cargo install`s.
    ///
    /// By default, whenever a call to `cargo install` fails for any reason,
    /// the overall operation is stopped as soon as possible. In some cases,
    /// such as packages simply failing to compile, this is a bit too
    /// restrictive as it prevents installing the following packages. The
    /// option it therefore provided in order to make the installation keep on
    /// going by continuing to call `cargo install` on each configured package,
    /// even if some previous one failed. However, in case any of the packages
    /// fails to install and the option is used, an error will still be
    /// reported at the end, containing an indication of all the packages that
    /// failed to install.
    #[arg(short, long)]
    pub keep_going: bool,

    /// Force overwriting existing crates or binaries.
    ///
    /// Passes the option flag onto each call of `cargo install`. It will, for
    /// example, redownload, recompile and reinstall every configured package
    /// when used in conjunction with `--skip-check`.
    #[arg(short, long)]
    pub force: bool,
}

#[derive(clap::Args, Debug, PartialEq, Eq)]
pub struct ImportArgs {
    /// Import package versions as "exact versions", i.e. prepended with
    /// an equal operator.
    ///
    /// Cannot be used in conjunction with either `--compatible` or `--patch`.
    /// Default: `false`, i.e. use a star requirement.
    #[arg(short, long)]
    #[arg(conflicts_with_all(["compatible", "patch"]))]
    pub exact: bool,

    /// Import package versions as "compatible versions", i.e. prepended with
    /// a caret operator.
    ///
    /// Cannot be used in conjunction with either `--exact` or `--patch`.
    /// Default: `false`, i.e. use a star requirement.
    #[arg(short, long)]
    #[arg(conflicts_with_all(["exact", "patch"]))]
    pub compatible: bool,

    /// Import package versions as "patch versions", i.e. prepended with
    /// a tilde operator.
    ///
    /// Cannot be used in conjunction with either `--exact` or `--compatible`.
    /// Default: `false`, i.e. use a star requirement.
    #[arg(short, long)]
    #[arg(conflicts_with_all(["exact", "compatible"]))]
    pub patch: bool,

    /// Overwrite the current configuration file if it already exists.
    ///
    /// Default: `false`, i.e. return an error in case the file already exists.
    #[arg(short, long)]
    pub force: bool,

    /// Also import this `cargo-liner` package into the configuration, for
    /// example in order to specify a certain version requirement later on.
    ///
    /// Default: `false`, i.e. exclude the current package from the list of
    /// packages to install or update in the resulting configuration file. Note
    /// however that the `ship` command will still self-update by default.
    #[arg(short = 's', long)]
    pub keep_self: bool,

    /// Also import all locally-installed packages into the configuration. This
    /// means packages installed via `cargo install --path <path>` will be
    /// present in the configuration.
    ///
    /// Default: `false`, i.e. exclude all packages installed via `cargo
    /// install --path <path>` from the list of packages to install or update
    /// in the resulting configuration file.
    #[arg(short = 'l', long)]
    pub keep_local: bool,
}

#[derive(clap::Args, Debug, PartialEq, Eq)]
pub struct CompletionsArgs {
    /// The shell flavor to use when generating the completions.
    pub shell: Shell,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_iserr() {
        assert!(CargoArgs::try_parse_from(["cargo", "liner", "--help"]).is_err());
    }

    #[test]
    fn test_version_iserr() {
        assert!(CargoArgs::try_parse_from(["cargo", "liner", "--version"]).is_err());
    }

    #[test]
    fn test_no_args() {
        let args = CargoArgs::try_parse_from(["cargo", "liner"]).unwrap();
        assert_eq!(
            args,
            CargoArgs::Liner(LinerArgs {
                command: None,
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
        assert_eq!(args.liner().verbosity(), 0);
    }

    #[allow(clippy::cast_possible_truncation)]
    #[test]
    fn test_verbose() {
        for i in 1..=5 {
            let args = CargoArgs::try_parse_from([
                "cargo",
                "liner",
                &("-".to_owned() + &str::repeat("v", i)),
            ])
            .unwrap();
            assert_eq!(
                args,
                CargoArgs::Liner(LinerArgs {
                    command: None,
                    verbose: i as u8,
                    quiet: 0,
                    color: ColorChoice::Auto,
                }),
            );
            assert_eq!(args.liner().verbosity(), i as i8);
        }
    }

    #[allow(clippy::cast_possible_truncation)]
    #[test]
    fn test_quiet() {
        for i in 1..=5 {
            let args = CargoArgs::try_parse_from([
                "cargo",
                "liner",
                &("-".to_owned() + &str::repeat("q", i)),
            ])
            .unwrap();
            assert_eq!(
                args,
                CargoArgs::Liner(LinerArgs {
                    command: None,
                    verbose: 0,
                    quiet: i as u8,
                    color: ColorChoice::Auto,
                }),
            );
            assert_eq!(args.liner().verbosity(), -(i as i8));
        }
    }

    #[test]
    fn test_quiet_verbose_iserr() {
        assert!(CargoArgs::try_parse_from(["cargo", "liner", "-qv"]).is_err());
    }

    #[test]
    fn test_color() {
        for (val, val_str) in [
            (ColorChoice::Always, "always"),
            (ColorChoice::Auto, "auto"),
            (ColorChoice::Never, "never"),
        ] {
            assert_eq!(
                CargoArgs::try_parse_from(["cargo", "liner", "--color", val_str]).unwrap(),
                CargoArgs::Liner(LinerArgs {
                    command: None,
                    verbose: 0,
                    quiet: 0,
                    color: val,
                }),
            );
        }
    }

    #[test]
    fn test_ship() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgs {
                    no_self: false,
                    only_self: false,
                    skip_check: false,
                    keep_going: false,
                    force: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_noselfupdate() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--no-self"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgs {
                    no_self: true,
                    only_self: false,
                    skip_check: false,
                    keep_going: false,
                    force: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_onlyselfupdate() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--only-self"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgs {
                    no_self: false,
                    only_self: true,
                    skip_check: false,
                    keep_going: false,
                    force: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_skipcheck() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--skip-check"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgs {
                    no_self: false,
                    only_self: false,
                    skip_check: true,
                    keep_going: false,
                    force: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_force() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--force"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgs {
                    no_self: false,
                    only_self: false,
                    skip_check: false,
                    keep_going: false,
                    force: true,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_noandonlyself_iserr() {
        assert!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--no-self", "--only-self"])
                .is_err()
        );
    }

    #[test]
    fn test_import() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: false,
                    force: false,
                    keep_self: false,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_force() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--force"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: false,
                    force: true,
                    keep_self: false,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_exact() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--exact"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: true,
                    compatible: false,
                    patch: false,
                    force: false,
                    keep_self: false,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_exact_force() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--exact", "--force"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: true,
                    compatible: false,
                    patch: false,
                    force: true,
                    keep_self: false,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_comp() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--compatible"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: true,
                    patch: false,
                    force: false,
                    keep_self: false,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_comp_force() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--compatible", "--force"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: true,
                    patch: false,
                    force: true,
                    keep_self: false,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_patch() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--patch"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: true,
                    force: false,
                    keep_self: false,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_patch_force() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--patch", "--force"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: true,
                    force: true,
                    keep_self: false,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_keepself() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--keep-self"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: false,
                    force: false,
                    keep_self: true,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_local() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--keep-local"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: false,
                    force: false,
                    keep_self: false,
                    keep_local: true,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_force_keepself() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--force", "--keep-self"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: false,
                    force: true,
                    keep_self: true,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_exact_keepself() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--exact", "--keep-self"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: true,
                    compatible: false,
                    patch: false,
                    force: false,
                    keep_self: true,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_exact_force_keepself() {
        assert_eq!(
            CargoArgs::try_parse_from([
                "cargo",
                "liner",
                "import",
                "--exact",
                "--force",
                "--keep-self"
            ])
            .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: true,
                    compatible: false,
                    patch: false,
                    force: true,
                    keep_self: true,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_comp_keepself() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--compatible", "--keep-self"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: true,
                    patch: false,
                    force: false,
                    keep_self: true,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_comp_force_keepself() {
        assert_eq!(
            CargoArgs::try_parse_from([
                "cargo",
                "liner",
                "import",
                "--compatible",
                "--force",
                "--keep-self"
            ])
            .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: true,
                    patch: false,
                    force: true,
                    keep_self: true,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_patch_keepself() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--patch", "--keep-self"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: true,
                    force: false,
                    keep_self: true,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_patch_force_keepself() {
        assert_eq!(
            CargoArgs::try_parse_from([
                "cargo",
                "liner",
                "import",
                "--patch",
                "--force",
                "--keep-self"
            ])
            .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: true,
                    force: true,
                    keep_self: true,
                    keep_local: false,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_import_exact_comp_iserr() {
        assert!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--exact", "--compatible"])
                .is_err()
        );
    }

    #[test]
    fn test_import_exact_patch_iserr() {
        assert!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--exact", "--patch"]).is_err()
        );
    }

    #[test]
    fn test_import_comp_patch_iserr() {
        assert!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--exact", "--compatible"])
                .is_err()
        );
    }

    #[test]
    fn test_completions() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "completions", "bash"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Completions(CompletionsArgs {
                    shell: Shell::Bash,
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            })
        );
    }

    #[test]
    fn test_completions_noarg_iserr() {
        assert!(CargoArgs::try_parse_from(["cargo", "liner", "completions"]).is_err());
    }

    #[test]
    fn test_completions_wrongarg_iserr() {
        assert!(
            CargoArgs::try_parse_from(["cargo", "liner", "completions", "idontexist"]).is_err()
        );
    }
}

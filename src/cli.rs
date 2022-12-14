//! Module handling all the CLI arguments configuration and parsing.
//!
//! See [`LinerArgs::parse_env`] in order to retrieve such arguments from the
//! environment.

use clap::Parser;

/// Cargo entry point for `cargo-liner`.
///
/// This tool is meant to be called using `cargo liner`.
#[derive(clap::Parser, Debug, PartialEq, Eq)]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoArgs {
    // The only variant: enables validating the input given by Cargo.
    Liner(LinerArgs),
}

// The actual entry point in this tool's argument parser.
#[derive(clap::Args, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about)]
pub struct LinerArgs {
    #[command(subcommand)]
    pub command: Option<LinerCommands>,
}

impl LinerArgs {
    /// Parses the arguments from the environment and returns them.
    ///
    /// Although it does not return `anyhow::Result<Self>`, the function is
    /// actually fallible: it will print an error to stderr and exit the current
    /// process on an error status code if a parsing error occurs.
    pub fn parse_env() -> Self {
        match CargoArgs::parse() {
            CargoArgs::Liner(args) => args,
        }
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
}

/// Arguments for the `ship` subcommand.
#[derive(clap::Args, Debug, PartialEq, Eq)]
#[command(long_about = None)]
pub struct ShipArgs {
    /// Disable self-updating.
    #[arg(long)]
    pub no_self_update: bool,
}

#[derive(clap::Args, Debug, PartialEq, Eq)]
pub struct ImportArgs {
    /// Import package versions as "exact versions", i.e. prepended with
    /// an equal operator.
    ///
    /// Cannot be used in conjunction with either `--compatible` or `--patch`.
    #[arg(short, long)]
    #[arg(conflicts_with_all(["compatible", "patch"]))]
    pub exact: bool,

    /// Import package versions as "compatible versions", i.e. prepended with
    /// a caret operator.
    ///
    /// Cannot be used in conjunction with either `--exact` or `--patch`.
    #[arg(short, long)]
    #[arg(conflicts_with_all(["exact", "patch"]))]
    pub compatible: bool,

    /// Import package versions as "patch versions", i.e. prepended with
    /// a tilde operator.
    ///
    /// Cannot be used in conjunction with either `--exact` or `--compatible`.
    #[arg(short, long)]
    #[arg(conflicts_with_all(["exact", "compatible"]))]
    pub patch: bool,

    /// Overwrite the current configuration file if it already exists.
    #[arg(short, long)]
    pub force: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_iserr() {
        assert!(CargoArgs::try_parse_from(["cargo", "liner", "--help"].into_iter()).is_err());
    }

    #[test]
    fn test_version_iserr() {
        assert!(CargoArgs::try_parse_from(["cargo", "liner", "--version"].into_iter()).is_err());
    }

    #[test]
    fn test_no_args() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner"].into_iter()).unwrap(),
            CargoArgs::Liner(LinerArgs { command: None }),
        );
    }

    #[test]
    fn test_ship() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship"].into_iter()).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgs {
                    no_self_update: false,
                })),
            }),
        );
    }

    #[test]
    fn test_ship_noselfupdate() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--no-self-update"].into_iter())
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgs {
                    no_self_update: true,
                })),
            }),
        );
    }

    #[test]
    fn test_import() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import"].into_iter()).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: false,
                    force: false,
                })),
            }),
        );
    }

    #[test]
    fn test_import_force() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--force"].into_iter()).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: false,
                    force: true,
                })),
            }),
        );
    }

    #[test]
    fn test_import_exact() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--exact"].into_iter()).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: true,
                    compatible: false,
                    patch: false,
                    force: false,
                })),
            }),
        );
    }

    #[test]
    fn test_import_exact_force() {
        assert_eq!(
            CargoArgs::try_parse_from(
                ["cargo", "liner", "import", "--exact", "--force"].into_iter()
            )
            .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: true,
                    compatible: false,
                    patch: false,
                    force: true,
                })),
            }),
        );
    }

    #[test]
    fn test_import_comp() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--compatible"].into_iter())
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: true,
                    patch: false,
                    force: false,
                })),
            }),
        );
    }

    #[test]
    fn test_import_comp_force() {
        assert_eq!(
            CargoArgs::try_parse_from(
                ["cargo", "liner", "import", "--compatible", "--force"].into_iter()
            )
            .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,

                    compatible: true,
                    patch: false,
                    force: true,
                })),
            }),
        );
    }

    #[test]
    fn test_import_patch() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "import", "--patch"].into_iter()).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: true,
                    force: false,
                })),
            }),
        );
    }

    #[test]
    fn test_import_patch_force() {
        assert_eq!(
            CargoArgs::try_parse_from(
                ["cargo", "liner", "import", "--patch", "--force"].into_iter()
            )
            .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Import(ImportArgs {
                    exact: false,
                    compatible: false,
                    patch: true,
                    force: true,
                })),
            }),
        );
    }

    #[test]
    fn test_import_exact_comp_iserr() {
        assert!(CargoArgs::try_parse_from(
            ["cargo", "liner", "import", "--exact", "--compatible"].into_iter()
        )
        .is_err());
    }

    #[test]
    fn test_import_exact_patch_iserr() {
        assert!(CargoArgs::try_parse_from(
            ["cargo", "liner", "import", "--exact", "--patch"].into_iter()
        )
        .is_err());
    }

    #[test]
    fn test_import_comp_patch_iserr() {
        assert!(CargoArgs::try_parse_from(
            ["cargo", "liner", "import", "--exact", "--compatible"].into_iter()
        )
        .is_err());
    }
}

//! Module handling all the CLI arguments configuration and parsing.
//!
//! See [`parse_args`] in order to retrieve such arguments from the environment.

use clap::Parser;

/// Cargo entry point for `cargo-liner`.
///
/// This tool is meant to be called using `cargo liner`.
#[derive(clap::Parser, Debug, PartialEq, Eq)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Cargo {
    // The only variant: enables validating the input given by Cargo.
    Liner(Liner),
}

// The actual entry point in this tool's argument parser.
#[derive(clap::Args, Debug, PartialEq, Eq)]
#[command(author, version, about, long_about)]
pub struct Liner {}

/// Parses the arguments from the environment and returns them.
///
/// Although it does not return `anyhow::Result<Liner>`, the function is
/// actually fallible: it will print an error to stderr and exit the current
/// process on an error status code if a parsing error occurs.
pub fn parse_args() -> Liner {
    match Cargo::parse() {
        Cargo::Liner(args) => args,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_help_iserr() {
        assert!(Cargo::try_parse_from(["cargo", "liner", "--help"].into_iter()).is_err());
    }

    #[test]
    fn test_version_iserr() {
        assert!(Cargo::try_parse_from(["cargo", "liner", "--version"].into_iter()).is_err());
    }

    #[test]
    fn test_no_args() {
        assert_eq!(
            Cargo::try_parse_from(["cargo", "liner"].into_iter()).unwrap(),
            Cargo::Liner(Liner {}),
        );
    }
}

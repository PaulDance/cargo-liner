//! Module handling all the CLI arguments configuration and parsing.
//!
//! See [`parse_args`] in order to retrieve such arguments from the environment.

use clap::Parser;

/// Argument parser that only accepts one subcommand: this tool.
///
/// Adapted from the [Clap Cookbook].
///
/// [Clap Cookbook]: https://docs.rs/clap/latest/clap/_derive/_cookbook/cargo_example_derive/index.html
#[derive(clap::Parser, Debug)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
enum Cargo {
    /// The only variant: enables validating the input given by Cargo.
    Liner(Liner),
}

/// The actual entry point in this tool's argument parser.
#[derive(clap::Args, Debug)]
#[command(author, version, about, long_about = None)]
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

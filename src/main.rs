//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
#![warn(unused_crate_dependencies)]

use std::{env, process};

use clap::ColorChoice;
use color_eyre::Section;
use color_eyre::config::{HookBuilder, Theme};
use color_eyre::eyre::{Result, WrapErr};
use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;
#[cfg(test)]
use tempfile as _;
#[cfg(test)]
use trycmd as _;

mod cargo;
mod cli;
use cli::{LinerArgs, LinerCommands, ShipArgs};
mod config;
use config::{EffectiveJettisonConfig, EffectiveShipConfig, UserConfig};
mod coloring;
use coloring::Colorizer;
mod commands;
#[cfg(test)]
#[path = "../tests/common/mod.rs"]
mod testing;

pub const OPEN_ISSUE_MSG: &str =
    "Open an issue using https://github.com/PaulDance/cargo-liner/issues/new/choose.";

/// Wrap the desired main and let `color-eyre` display errors.
fn main() -> Result<()> {
    // More user-guiding panics in release builds.
    human_panic::setup_panic!();
    // Logging and error reporting are controlled by the CLI arguments, so they
    // must be parsed first.
    let args = LinerArgs::parse_env();

    // Let the CLI verbosity have control over the backtraces displayed.
    if let verb @ 1.. = args.verbosity() {
        // SAFETY: no other thread can exist at this early point.
        unsafe {
            env::set_var(
                "RUST_BACKTRACE",
                match verb {
                    1 => "1",
                    _ => "full",
                },
            );
        }
    }

    install_error_hook(&args)?;
    // HACK: reproduce the previous behavior by directly exiting: don't display
    // anything, but only report an error code when verbosity is low enough.
    try_main(&args).inspect_err(|_| {
        if args.verbosity() <= -3 {
            process::exit(1);
        }
    })
}

/// Initializes the `color-eyre` machinery.
///
/// Explicitely disables error colors when explicitely asked to do so in the
/// passed CLI arguments.
fn install_error_hook(args: &LinerArgs) -> Result<()> {
    if args.color == ColorChoice::Never {
        HookBuilder::new().theme(Theme::new()).install()
    } else {
        color_eyre::install()
    }
    .wrap_err("Failed to install the error hook.")
    .note("This should only happen if it is attempted twice.")
    .suggestion(OPEN_ISSUE_MSG)
}

/// Actual main operation.
fn try_main(args: &LinerArgs) -> Result<()> {
    init_logger(args)?;
    let colorizer = Colorizer::new(&std::io::stderr(), args.color);
    let cargo_verbosity = match args.verbosity() {
        -2..=1 => 0,
        v if v > 1 => v - 1,
        q if q < -2 => q + 2,
        _ => unreachable!(),
    };

    // CLI command dispatch.
    match &args.command {
        Some(LinerCommands::Completions(comp_args)) => {
            commands::completions::run(comp_args);
        }
        Some(LinerCommands::Import(import_args)) => {
            commands::import::run(import_args)?;
        }
        Some(LinerCommands::Jettison(jettison_args)) => {
            commands::jettison::run(
                &EffectiveJettisonConfig::new(
                    UserConfig::parse_file().wrap_err("Failed to parse the user configuration.")?,
                    config::env::jettison_env_args()
                        .wrap_err("Failed to get one of the environment variables.")?,
                    jettison_args.as_ref().to_owned(),
                ),
                *colorizer.color(),
                cargo_verbosity,
            )?;
        }
        cmd @ (None | Some(LinerCommands::Ship(_))) => {
            commands::ship::run(
                &EffectiveShipConfig::new(
                    UserConfig::parse_file().wrap_err("Failed to parse the user configuration.")?,
                    config::env::ship_env_args()
                        .wrap_err("Failed to get one of the environment variables.")?,
                    if let Some(LinerCommands::Ship(ship_args)) = cmd {
                        ship_args.as_ref().to_owned()
                    } else {
                        ShipArgs::default()
                    },
                ),
                &colorizer,
                cargo_verbosity,
            )?;
        }
    }

    log::info!("Done.");
    Ok(())
}

/// Initializes the logger machinery form the passed CLI arguments.
fn init_logger(args: &LinerArgs) -> Result<()> {
    let mut bld = pretty_env_logger::formatted_builder();
    bld.parse_default_env();

    // Logging setup parameterized by CLI args.
    if args.verbosity().abs() < 2 {
        bld.filter_module("::", LevelFilter::Error);
        bld.filter_module(
            env!("CARGO_CRATE_NAME"),
            match args.verbosity() {
                0 => LevelFilter::Info,
                1 => LevelFilter::Debug,
                -1 => LevelFilter::Warn,
                _ => unreachable!(),
            },
        );
    } else {
        bld.filter_level(match args.verbosity() {
            2 => LevelFilter::Debug,
            v if v > 2 => LevelFilter::Trace,
            -2 => LevelFilter::Error,
            q if q < -2 => LevelFilter::Off,
            _ => unreachable!(),
        });
    }
    bld.write_style(match args.color {
        ColorChoice::Always => WriteStyle::Always,
        ColorChoice::Auto => WriteStyle::Auto,
        ColorChoice::Never => WriteStyle::Never,
    });
    bld.try_init()
        .wrap_err("Failed to initialize the logger.")
        .note("This should only happen if it is attempted twice.")
        .suggestion(OPEN_ISSUE_MSG)
}

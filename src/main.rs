//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
#![warn(unused_crate_dependencies)]

use std::collections::{BTreeMap, BTreeSet};
use std::{env, process};

use clap::ColorChoice;
use color_eyre::Section;
use color_eyre::config::{HookBuilder, Theme};
use color_eyre::eyre::{Result, WrapErr};
use log::LevelFilter;
use pretty_env_logger::env_logger::WriteStyle;
use semver::Version;
use tabled::settings::Style;
use tabled::{Table, Tabled};
#[cfg(test)]
use tempfile as _;
#[cfg(test)]
use trycmd as _;

mod cargo;
use cargo::InstallStatus;
mod cli;
use cli::{LinerArgs, LinerCommands, ShipArgs};
mod config;
use config::{CargoCratesToml, DetailedPackageReq, EffectiveConfig, UserConfig};
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

    // CLI command dispatch.
    match &args.command {
        Some(LinerCommands::Completions(comp_args)) => {
            commands::completions::run(comp_args);
        }
        Some(LinerCommands::Import(import_args)) => {
            commands::import::run(import_args)?;
        }
        cmd @ (None | Some(LinerCommands::Ship(_))) => {
            let cargo_verbosity = match args.verbosity() {
                -2..=1 => 0,
                v if v > 1 => v - 1,
                q if q < -2 => q + 2,
                _ => unreachable!(),
            };
            let config = EffectiveConfig::new(
                UserConfig::parse_file().wrap_err("Failed to parse the user configuration.")?,
                config::env::ship_env_args()
                    .wrap_err("Failed to get one of the environment variables.")?,
                if let Some(LinerCommands::Ship(ship_args)) = cmd {
                    ship_args.clone().into_inner()
                } else {
                    ShipArgs::default()
                },
            );

            let (inst_res, old_vers, new_vers) = if config.ship_args.skip_check {
                // Don't parse `.crates.toml` here: can be used as a workaround.
                (
                    cargo::install_all(
                        &config.packages,
                        &BTreeSet::new(),
                        config.ship_args.no_fail_fast,
                        config.ship_args.force,
                        config.ship_args.dry_run,
                        config.ship_args.binstall,
                        args.color,
                        cargo_verbosity,
                    ),
                    BTreeMap::new(),
                    BTreeMap::new(),
                )
            } else {
                let cct = CargoCratesToml::parse_file()
                    .wrap_err("Failed to parse Cargo's .crates.toml file.")?;
                let old_vers = cct.clone().into_name_versions();
                let new_vers = cargo::search_exact_all(
                    &config
                        .packages
                        .iter()
                        .filter(|(_, pkg)| !pkg.effective_skip_check())
                        .map(|(name, _)| name.clone())
                        .collect::<Vec<_>>(),
                )
                .wrap_err("Failed to fetch the latest versions of the configured packages.")?;
                log_version_check_summary(&colorizer, &config.packages, &new_vers, &old_vers);

                (
                    cargo::install_all(
                        &needing_install(&config.packages, &new_vers, &old_vers),
                        &cct.into_names(),
                        config.ship_args.no_fail_fast,
                        config.ship_args.force,
                        config.ship_args.dry_run,
                        config.ship_args.binstall,
                        args.color,
                        cargo_verbosity,
                    ),
                    old_vers,
                    new_vers,
                )
            };

            if let Some(err) = match inst_res {
                Ok(rep) => {
                    log_install_report(
                        &colorizer,
                        &rep.package_statuses,
                        &new_vers,
                        &old_vers,
                        config.ship_args.dry_run,
                    );
                    rep.error_report
                }
                Err(err) => Some(err),
            } {
                Err(err).wrap_err_with(|| {
                    format!(
                        "Failed to install or update {} of the configured packages.",
                        if config.ship_args.no_fail_fast {
                            "some"
                        } else {
                            "one"
                        }
                    )
                })?;
            }
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

/// Returns the packages that do indeed need an install or update.
fn needing_install(
    pkgs: &BTreeMap<String, DetailedPackageReq>,
    new_vers: &BTreeMap<String, Version>,
    old_vers: &BTreeMap<String, Version>,
) -> BTreeMap<String, DetailedPackageReq> {
    let mut to_install = BTreeMap::new();
    log::debug!("Filtering packages by versions...");

    for (pkg_name, pkg) in pkgs {
        if pkg.effective_skip_check()
            || old_vers
                .get(pkg_name)
                // UNWRAP: `pkg.effective_skip_check()` was just checked for.
                .is_none_or(|ver| ver < new_vers.get(pkg_name).unwrap())
        {
            to_install.insert(pkg_name.clone(), pkg.clone());
            log::trace!("{pkg_name:?} is selected to be installed or updated.");
        } else {
            log::trace!("{pkg_name:?} is not selected: already up-to-date.");
        }
    }

    log::trace!("Filtered packages: {to_install:#?}.");
    to_install
}

#[derive(Tabled)]
struct PackageStatus {
    #[tabled(rename = "Name")]
    name: String,
    #[tabled(rename = "Old version")]
    old_ver: String,
    #[tabled(rename = "New version")]
    new_ver: String,
    #[tabled(rename = "Status")]
    status: String,
}

/// Displays whether each package needs an update or not.
fn log_version_check_summary(
    colorizer: &Colorizer,
    pkg_reqs: &BTreeMap<String, DetailedPackageReq>,
    new_vers: &BTreeMap<String, Version>,
    old_vers: &BTreeMap<String, Version>,
) {
    if pkg_reqs.is_empty() {
        log::info!("No package to install: none was configured and self was skipped.");
    } else {
        log::info!(
            "Results:\n{}",
            // Iterate on `pkg_reqs` in order to get the package names: due to
            // the possibility of a partial `skip-check`, `old_vers U new_vers`
            // may not contain every one of them.
            Table::new(pkg_reqs.keys().map(|pkg_name| {
                let old_ver = old_vers.get(pkg_name);
                let new_ver = new_vers.get(pkg_name);
                PackageStatus {
                    name: pkg_name.clone(),
                    old_ver: old_ver
                        .map_or_else(|| colorizer.none_icon().to_string(), ToString::to_string),
                    // This should be equivalent to checking `effective_skip_check`:
                    // a new version is unavailable only if not fetched.
                    new_ver: new_ver.map_or_else(
                        || colorizer.unknown_icon().to_string(),
                        |new_ver| {
                            old_ver
                                .and_then(|old_ver| {
                                    (old_ver >= new_ver).then(|| colorizer.none_icon().to_string())
                                })
                                .unwrap_or_else(|| new_ver.to_string())
                        },
                    ),
                    status: new_ver
                        .and_then(|new_ver| {
                            old_ver.and_then(|old_ver| {
                                (old_ver >= new_ver).then(|| colorizer.ok_icon().to_string())
                            })
                        })
                        .unwrap_or_else(|| colorizer.todo_icon().to_string()),
                }
            }))
            .with(Style::sharp())
        );
    }
}

/// Displays the ending report about each package's installation status.
fn log_install_report(
    colorizer: &Colorizer,
    install_report: &BTreeMap<String, InstallStatus>,
    new_vers: &BTreeMap<String, Version>,
    old_vers: &BTreeMap<String, Version>,
    dry_run: bool,
) {
    if !install_report.is_empty() {
        log::info!(
            "Installation report:\n{}",
            Table::new(install_report.iter().map(|(pkg_name, status)| {
                PackageStatus {
                    name: pkg_name.clone(),
                    old_ver: old_vers
                        .get(pkg_name)
                        .map_or_else(|| colorizer.none_icon().to_string(), ToString::to_string),
                    new_ver: new_vers
                        .get(pkg_name)
                        .map_or_else(|| colorizer.unknown_icon().to_string(), ToString::to_string),
                    status: match status {
                        InstallStatus::Installed => colorizer.new_icon().to_string(),
                        InstallStatus::Updated => colorizer.ok_icon().to_string(),
                        InstallStatus::Failed => colorizer.err_icon().to_string(),
                    },
                }
            }))
            .with(Style::sharp())
        );

        if dry_run {
            log::warn!("This is a dry run, so this report is simulated.");
        }
    }
}

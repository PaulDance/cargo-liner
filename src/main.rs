//! Main module: regroups parsing CLI arguments, deserializing configuration,
//! and execution of `cargo install` with the required settings.
#![warn(unused_crate_dependencies)]

use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Display;
use std::io::IsTerminal;
use std::{env, io, process};

use clap::{ColorChoice, CommandFactory};
use color_eyre::Section;
use color_eyre::config::{HookBuilder, Theme};
use color_eyre::eyre::{self, Result, WrapErr};
use color_eyre::owo_colors::OwoColorize;
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
#[expect(
    clippy::too_many_lines,
    reason = "This the main part, so contains all supported commands and their respective processing."
)]
fn try_main(args: &LinerArgs) -> Result<()> {
    init_logger(args)?;
    let colorizer = Colorizer::new(&std::io::stderr(), args.color);

    // CLI command dispatch.
    match &args.command {
        Some(LinerCommands::Completions(comp_args)) => {
            log::info!(
                "Generating auto-completion script for {:?}...",
                comp_args.shell,
            );
            clap_complete::generate(
                comp_args.shell,
                &mut LinerArgs::command(),
                clap::crate_name!(),
                &mut io::stdout(),
            );
        }
        Some(LinerCommands::Import(import_args)) => {
            if UserConfig::file_path()
                .wrap_err("Failed to build the configuration file path.")?
                .try_exists()
                .wrap_err("Failed to check if the configuration file exists.")
                .suggestion("Check the permissions of the Cargo home directory.")?
            {
                if import_args.force {
                    log::warn!("Configuration file will be overwritten.");
                } else {
                    eyre::bail!("Configuration file already exists, use -f/--force to overwrite.");
                }
            }
            log::info!("Importing Cargo installed crates as a new configuration file...");
            // Clap conflict settings ensure the options are mutually exclusive.
            (if import_args.force {
                UserConfig::overwrite_file
            } else {
                UserConfig::save_file
            })(&(if import_args.exact {
                CargoCratesToml::into_exact_version_config
            } else if import_args.compatible {
                CargoCratesToml::into_comp_version_config
            } else if import_args.patch {
                CargoCratesToml::into_patch_version_config
            } else {
                CargoCratesToml::into_star_version_config
            })(
                CargoCratesToml::parse_file()
                    .wrap_err("Failed to parse Cargo's .crates.toml file.")?,
                import_args.keep_self,
                import_args.keep_local,
            ))
            .wrap_err("Failed to save the configuration file.")?;
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
                let new_vers = cargo::search_exact_all(&config.packages)
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
        if pkg.skip_check
            || old_vers
                .get(pkg_name)
                // UNWRAP: `pkg.skip_check` was just checked for.
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
    if new_vers.is_empty() {
        log::info!("No packages installed.");
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
                    // This should be equivalent to checking `pkg_req.skip_check`:
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

/// Collection of adequate characters to be used as display icons.
mod icons {
    /// When nothing to display or needs to be done: already up-to-date.
    pub(super) const NONE: char = 'ø';
    /// When the element could not be determined, for example the new version
    /// of a package if `skip-check` is used.
    pub(super) const UNKNOWN: char = '?';
    /// When something needs to be performed: installation or update of a
    /// package.
    pub(super) const TODO: char = '🛈';
    /// When something was successfully added: new installation of a package.
    pub(super) const NEW: char = '+';
    /// When something failed.
    pub(super) const ERR: char = '✘';
    /// When things went right: already up-to-date or successful update.
    pub(super) const OK: char = '✔';
}

/// Assembles both an output stream's color capacity and a color preference in
/// order to condtionally emit colorized content.
struct Colorizer {
    is_terminal: bool,
    color_choice: ColorChoice,
}

impl Colorizer {
    /// Builds a new colorizer.
    ///
    ///  * `out`: the descriptor that will be used in order to write the output
    ///    of [`Self::colorize_with`].
    ///  * `color_choice`: color preference to apply.
    pub fn new<D: IsTerminal>(out: &D, color_choice: ColorChoice) -> Self {
        Self {
            is_terminal: out.is_terminal(),
            color_choice,
        }
    }

    /// Returns `input` or `color_fn(input)` depending on the current color
    /// preference and whether a terminal is used.
    pub fn colorize_with<'i, 'o, I, F, O>(&self, input: &'i I, color_fn: F) -> Box<dyn Display + 'o>
    where
        'i: 'o,
        I: Display + ?Sized,
        O: Display + 'o,
        F: Fn(&'i I) -> O,
    {
        match self.color_choice {
            ColorChoice::Never => Box::new(input),
            ColorChoice::Always => Box::new(color_fn(input)),
            ColorChoice::Auto => {
                if self.is_terminal {
                    Box::new(color_fn(input))
                } else {
                    Box::new(input)
                }
            }
        }
    }

    /// Returns the colorized version of [`icons::NONE`].
    #[expect(
        clippy::unused_self,
        reason = "So refactors may be easier by keeping an API identical to the other icons."
    )]
    pub fn none_icon(&self) -> impl Display {
        icons::NONE
    }

    /// Returns the colorized version of [`icons::UNKNOWN`].
    pub fn unknown_icon(&self) -> impl Display {
        self.colorize_with(&icons::UNKNOWN, |icon| icon.bold().yellow().to_string())
    }

    /// Returns the colorized version of [`icons::TODO`].
    pub fn todo_icon(&self) -> impl Display {
        self.colorize_with(&icons::TODO, |icon| icon.bold().blue().to_string())
    }

    /// Returns the colorized version of [`icons::NEW`].
    pub fn new_icon(&self) -> impl Display {
        self.colorize_with(&icons::NEW, |icon| icon.bold().green().to_string())
    }

    /// Returns the colorized version of [`icons::ERR`].
    pub fn err_icon(&self) -> impl Display {
        self.colorize_with(&icons::ERR, char::red)
    }

    /// Returns the colorized version of [`icons::OK`].
    pub fn ok_icon(&self) -> impl Display {
        self.colorize_with(&icons::OK, char::green)
    }
}

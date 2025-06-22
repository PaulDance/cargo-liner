//! Module handling all the CLI arguments configuration and parsing.
//!
//! See [`LinerArgs::parse_env`] in order to retrieve such arguments from the
//! environment.
#![expect(
    clippy::struct_excessive_bools,
    reason = "This is the CLI module, so contains types handling all supported boolean flags."
)]
use std::str::FromStr;

use clap::builder::ArgPredicate;
use clap::{ArgAction, ColorChoice, Parser, ValueEnum};
use clap_complete::Shell;
use serde::{Deserialize, Serialize};

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
    /// `-v` is given to Cargo calls (details ran commands), `--log-level debug`
    /// is given to `cargo-binstall` when using it, and error backtraces are
    /// fully shown (`RUST_BACKTRACE=full`).
    /// When used three times or more, TRACE and above messages of all crates
    /// are logged, `-vv` is given to Cargo calls (includes build output),
    /// `--log-level trace` is given to `cargo-binstall` when using it, and
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
    /// When used once, WARN and above messages of only this crate are logged,
    /// and `--log-level warn` is given to `cargo-binstall` when using it.
    /// When used twice, ERROR messages of all crates are logged, and
    /// `--log-level error` is given to `cargo-binstall` when using it.
    /// When used three times or more, no message will be logged, including
    /// Cargo's by passing `-q` to it and `cargo-binstall`'s by passing
    /// `--log-level off` to it, and error reports are silenced.
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
    /// calls to Cargo, but not `cargo-binstall` when using it as it does not
    /// yet have any similar option.
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
    Ship(ShipArgsWithNegations),

    /// Uninstall not-configured packages.
    ///
    /// This will remove all packages that are currently installed according to
    /// Cargo, but that are not part of the Liner configuration. Use with care!
    Jettison(JettisonArgsWithNegations),

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
#[derive(clap::Args, Serialize, Deserialize, Debug, Default, Clone, PartialEq, Eq)]
#[serde(default, rename_all = "kebab-case")]
pub struct ShipArgs {
    /// Disable self-updating.
    ///
    /// Cannot be used in conjunction with `--only-self`.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_SHIP_NO_SELF`]
    ///
    /// [config: `defaults.ship.no-self`]
    #[arg(
        short,
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_with_self", ArgPredicate::IsPresent, "false"),
        conflicts_with("only_self"),
        display_order = 1
    )]
    pub no_self: Option<bool>,

    /// Only self-update and do not install or update any other package.
    ///
    /// Cannot be used in conjunction with `--no-self`.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_SHIP_ONLY_SELF`]
    ///
    /// [config: `defaults.ship.only-self`]
    #[arg(
        short = 's',
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_no_only_self", ArgPredicate::IsPresent, "false"),
        conflicts_with("no_self"),
        display_order = 3
    )]
    pub only_self: Option<bool>,

    /// Skip the summary version check and directly call `cargo install` or
    /// `cargo binstall` on each configured package.
    ///
    /// The version check is relatively quick and enables skipping calls to
    /// `cargo install` or `cargo binstall` when no update is required, which
    /// saves quite a bit of time. However, if you wish, this option is
    /// still available in order not to run the check: doing so will
    /// probably take more time in the end most of the time, except if you
    /// have a very small amount of packages configured (e.g. one or two) or
    /// if all or almost all packages are not already installed.
    ///
    /// It can also be used as a workaround in case a certain operation fails
    /// in your particular environment, for example: reading from `.crates.toml`
    /// under the `$CARGO_HOME` or `$CARGO_INSTALL_ROOT` directory or making
    /// requests to the registry. These operations will thus be entirely
    /// skipped.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_SHIP_SKIP_CHECK`]
    ///
    /// [config: `defaults.ship.skip-check`]
    #[arg(
        short = 'c',
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_no_skip_check", ArgPredicate::IsPresent, "false"),
        display_order = 5
    )]
    pub skip_check: Option<bool>,

    /// Disable the default fail-fast execution of `cargo install`s.
    ///
    /// By default, whenever a call to `cargo install` or `cargo binstall` fails
    /// for any reason, the overall operation is stopped as soon as
    /// possible. In some cases, such as packages simply failing to compile,
    /// this is a bit too restrictive as it prevents installing the
    /// following packages. The option it therefore provided in order to
    /// make the installation keep on going by continuing to call `cargo
    /// install` on each configured package, even if some previous one
    /// failed. However, in case any of the packages fails to install and
    /// the option is used, an error will still be reported at the end,
    /// containing an indication of all the packages that failed to install.
    ///
    /// This is not to be confused with Cargo's `--keep-going` build option: it
    /// disables fast-failing between crate compilations, while the current one
    /// disables fast-failing between entire calls to `cargo install` or `cargo
    /// binstall`; in fact, `--keep-going` is never passed onto Cargo. It is
    /// neither to be confused with `cargo test --no-fail-fast` since `cargo
    /// test` is never used.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_SHIP_NO_FAIL_FAST`]
    ///
    /// [config: `defaults.ship.no-fail-fast`]
    #[arg(
        short = 'k',
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_fail_fast", ArgPredicate::IsPresent, "false"),
        display_order = 7
    )]
    pub no_fail_fast: Option<bool>,

    /// Force overwriting existing crates or binaries.
    ///
    /// Passes the option flag onto each call of `cargo install` or `cargo
    /// binstall`. It will, for example, redownload, recompile and reinstall
    /// every configured package when used in conjunction with
    /// `--skip-check`.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_SHIP_FORCE`]
    ///
    /// [config: `defaults.ship.force`]
    #[arg(
        short,
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_no_force", ArgPredicate::IsPresent, "false"),
        display_order = 9
    )]
    pub force: Option<bool>,

    /// Perform all operations without actually installing.
    ///
    /// This disables any installation step and replaces them with simulations,
    /// but retains all the remaining operations. This may be useful in order
    /// to observe what would be performed without actually doing it. In
    /// particular, it may serve as a quicker way to check if new versions are
    /// available or not.
    ///
    /// Currently, `cargo install --dry-run` is not stabilized yet, so the
    /// option is not passed onto such calls. However, `cargo-binstall` has
    /// such an option, so it is passed onto its calls whenever it is used.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_SHIP_DRY_RUN`]
    ///
    /// [config: `defaults.ship.dry-run`]
    #[arg(
        short,
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_no_dry_run", ArgPredicate::IsPresent, "false"),
        display_order = 11
    )]
    pub dry_run: Option<bool>,

    /// Control the usage of `cargo-binstall`.
    ///
    /// This third-party tool has dedicated support here. It is meant to be
    /// optional and easily pluggable, however, hence the chosen default. When
    /// it is enabled, it effectively replaces `cargo install` entirely and all
    /// compatible options are forwarded to it.
    ///
    /// [default: auto]
    ///
    /// [env: `CARGO_LINER_SHIP_BINSTALL`]
    ///
    /// [config: `defaults.ship.binstall`]
    #[arg(
        short,
        long,
        required = false,
        value_enum,
        value_name = "BINSTALL_WHEN",
        display_order = 50
    )]
    pub binstall: Option<BinstallChoice>,
}

/// Choices for [`ShipArgs::binstall`].
#[derive(Serialize, Deserialize, clap::ValueEnum, Debug, Clone, Copy, Default, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum BinstallChoice {
    /// The tool is heuristically detected and used if available.
    ///
    /// Currently, the heuristics are the following:
    ///  * if default options are used, then the tool is considered as available
    ///    as soon as it is seen in the list of currently-installed packages
    ///    that is read as part of the usual operation;
    ///  * if `--skip-check` is used, then a direct call to the tool is
    ///    attempted through Cargo and it is considered available if everything
    ///    succeeds, since the list of installed packages is not read here.
    #[default]
    Auto,
    /// Always attempt to use it without trying to detect it first.
    Always,
    /// Completely disable the feature and only rely on Cargo.
    Never,
}

/// Clone of [`ColorChoice::from_str`].
impl FromStr for BinstallChoice {
    type Err = clap::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            // UNWRAP: no value is skipped.
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        // HACK: use this to get `std::error::Error` for free.
        Err(clap::Command::new(clap::crate_name!()).error(
            clap::error::ErrorKind::InvalidValue,
            format!("Invalid variant: {s:?}"),
        ))
    }
}

/// Regroupement of flags from [`ShipArgs`] with their negated couterparts.
#[derive(clap::Args, Debug, Default, Clone, PartialEq, Eq)]
pub struct ShipArgsWithNegations {
    /// The actual arguments.
    #[command(flatten)]
    inner: ShipArgs,

    /// Negation of `--no-self` that overrides it and restores the default
    /// behavior as if absent, i.e. self-update.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "no_self",
        display_order = 2
    )]
    _with_self: (),

    /// Negation of `--only-self` that overrides it and restores the default
    /// behavior as if absent, i.e. install or update other packages as well.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "only_self",
        display_order = 4
    )]
    _no_only_self: (),

    /// Negation of `--skip-check` that overrides it and restores the default
    /// behavior as if absent, i.e. perform the usual version check.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "skip_check",
        display_order = 6
    )]
    _no_skip_check: (),

    /// Negation of `--no-fail-fast` that overrides it and restores the default
    /// behavior as if absent, i.e. stop as soon as the first error occurs.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "no_fail_fast",
        display_order = 8
    )]
    _fail_fast: (),

    /// Negation of `--force` that overrides it and restores the default
    /// behavior as if absent, i.e. don't pass the argument onto Cargo.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "force",
        display_order = 10
    )]
    _no_force: (),

    /// Negation of `--dry-run` that overrides it and restores the default
    /// behavior as if absent, i.e. perform the installations as per the usual.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "dry_run",
        display_order = 12
    )]
    _no_dry_run: (),
}

impl AsRef<ShipArgs> for ShipArgsWithNegations {
    /// Returns a reference to the wrapped [`ShipArgs`].
    fn as_ref(&self) -> &ShipArgs {
        &self.inner
    }
}

/// Arguments for the `jettison` subcommand.
#[derive(clap::Args, Serialize, Deserialize, Debug, Clone, Default, PartialEq, Eq)]
#[serde(default, rename_all = "kebab-case")]
pub struct JettisonArgs {
    /// Disable the confirmation of removal.
    ///
    /// By default, an interactive confirmation is prompted to the user in
    /// order to avoid hasty deletions, proceeding only when either no input is
    /// given (enter key directly) or `y` is entered. If no standard input is
    /// available, which should for example be the case in CI or server
    /// environments where there is no tty setup, then the confirmaton is
    /// immediately passed. In any case, this flag always disables it entirely.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_JETTISON_NO_CONFIRM`]
    ///
    /// [config: `defaults.jettison.no-confirm`]
    #[arg(
        short = 'y',
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_confirm", ArgPredicate::IsPresent, "false"),
        display_order = 1
    )]
    pub no_confirm: Option<bool>,

    /// Disable the default fail-fast execution of `cargo uninstall`s.
    ///
    /// By default, whenever a call to `cargo uninstall` fails for any reason,
    /// the overall operation is stopped as soon as possible. In some cases,
    /// this is a bit too restrictive as it prevents uninstalling the following
    /// packages. The option it therefore provided in order to make the
    /// uninstallation keep on going by continuing to call `cargo uninstall` on
    /// each concerned package, even if some previous one failed. However, in
    /// case any of the packages fails to uninstall and the option is used, an
    /// error will still be reported at the end, containing an indication of all
    /// the packages that failed to uninstall.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_JETTISON_NO_FAIL_FAST`]
    ///
    /// [config: `defaults.jettison.no-fail-fast`]
    #[arg(
        short = 'k',
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_fail_fast", ArgPredicate::IsPresent, "false"),
        display_order = 3
    )]
    pub no_fail_fast: Option<bool>,

    /// Perform all operations without actually uninstalling.
    ///
    /// This disables any uninstallation step and replaces them with
    /// simulations, but retains all the remaining operations. This may be
    /// useful in order to observe what would be performed without actually
    /// doing it. This implies `--no-confirm`.
    ///
    /// [default: false]
    ///
    /// [env: `CARGO_LINER_JETTISON_DRY_RUN`]
    ///
    /// [config: `defaults.jettison.dry-run`]
    #[arg(
        short = 'n',
        long,
        num_args = 0,
        default_missing_value = "true",
        default_value_if("_no_dry_run", ArgPredicate::IsPresent, "false"),
        display_order = 5
    )]
    pub dry_run: Option<bool>,
}

/// Regroupement of flags from [`JettisonArgs`] with their negated couterparts.
#[derive(clap::Args, Debug, PartialEq, Eq)]
pub struct JettisonArgsWithNegations {
    /// The actual arguments.
    #[command(flatten)]
    inner: JettisonArgs,

    /// Negation of `--no-confirm` that overrides it and restores the default
    /// behavior as if absent, i.e. ask for confirmation before proceeding.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "no_confirm",
        display_order = 2
    )]
    _confirm: (),

    /// Negation of `--no-fail-fast` that overrides it and restores the default
    /// behavior as if absent, i.e. stop as soon as the first error occurs.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "no_fail_fast",
        display_order = 4
    )]
    _fail_fast: (),

    /// Negation of `--dry-run` that overrides it and restores the default
    /// behavior as if absent, i.e. perform the uninstallations as per the
    /// usual.
    #[arg(
        long,
        required = false,
        num_args = 0,
        overrides_with = "dry_run",
        display_order = 6
    )]
    _no_dry_run: (),
}

impl AsRef<JettisonArgs> for JettisonArgsWithNegations {
    /// Returns a reference to the wrapped [`JettisonArgs`].
    fn as_ref(&self) -> &JettisonArgs {
        &self.inner
    }
}

/// Arguments for the `import` subcommand.
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

/// Arguments for the `completions` subcommand.
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

    #[expect(
        clippy::cast_possible_truncation,
        reason = "The concerned integers are too small for any truncation to ever be possible."
    )]
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

    #[expect(
        clippy::cast_possible_truncation,
        reason = "The concerned integers are too small for any truncation to ever be possible."
    )]
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
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_noself() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--no-self"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: Some(true),
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_noself_negation() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--no-self", "--with-self"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: Some(false),
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_onlyself() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--only-self"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: Some(true),
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_onlyself_negation() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--only-self", "--no-only-self"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: Some(false),
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
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
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: Some(true),
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_skipcheck_negation() {
        assert_eq!(
            CargoArgs::try_parse_from([
                "cargo",
                "liner",
                "ship",
                "--skip-check",
                "--no-skip-check",
            ])
            .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: Some(false),
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_nofailfast() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--no-fail-fast"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: Some(true),
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_nofailfast_negation() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--no-fail-fast", "--fail-fast"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: Some(false),
                        force: None,
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
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
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: Some(true),
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_force_negation() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--force", "--no-force"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: Some(false),
                        dry_run: None,
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_dryrun() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--dry-run"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: Some(true),
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_dryrun_negation() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--dry-run", "--no-dry-run"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: Some(false),
                        binstall: None,
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_binstall_auto() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--binstall", "auto"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: Some(BinstallChoice::Auto),
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_binstall_always() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--binstall", "always"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: Some(BinstallChoice::Always),
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_ship_binstall_never() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "ship", "--binstall", "never"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Ship(ShipArgsWithNegations {
                    inner: ShipArgs {
                        no_self: None,
                        only_self: None,
                        skip_check: None,
                        no_fail_fast: None,
                        force: None,
                        dry_run: None,
                        binstall: Some(BinstallChoice::Never),
                    },
                    _with_self: (),
                    _no_only_self: (),
                    _no_skip_check: (),
                    _fail_fast: (),
                    _no_force: (),
                    _no_dry_run: (),
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
    fn test_jettison_noconfirm() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "jettison", "--no-confirm"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Jettison(JettisonArgsWithNegations {
                    inner: JettisonArgs {
                        no_confirm: Some(true),
                        no_fail_fast: None,
                        dry_run: None,
                    },
                    _confirm: (),
                    _fail_fast: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_jettison_noconfirm_negation() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "jettison", "--no-confirm", "--confirm"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Jettison(JettisonArgsWithNegations {
                    inner: JettisonArgs {
                        no_confirm: Some(false),
                        no_fail_fast: None,
                        dry_run: None,
                    },
                    _confirm: (),
                    _fail_fast: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_jettison_nofailfast() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "jettison", "--no-fail-fast"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Jettison(JettisonArgsWithNegations {
                    inner: JettisonArgs {
                        no_confirm: None,
                        no_fail_fast: Some(true),
                        dry_run: None,
                    },
                    _confirm: (),
                    _fail_fast: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_jettison_nofailfast_negation() {
        assert_eq!(
            CargoArgs::try_parse_from([
                "cargo",
                "liner",
                "jettison",
                "--no-fail-fast",
                "--fail-fast"
            ])
            .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Jettison(JettisonArgsWithNegations {
                    inner: JettisonArgs {
                        no_confirm: None,
                        no_fail_fast: Some(false),
                        dry_run: None,
                    },
                    _confirm: (),
                    _fail_fast: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_jettison_dryrun() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "jettison", "--dry-run"]).unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Jettison(JettisonArgsWithNegations {
                    inner: JettisonArgs {
                        no_confirm: None,
                        no_fail_fast: None,
                        dry_run: Some(true),
                    },
                    _confirm: (),
                    _fail_fast: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
        );
    }

    #[test]
    fn test_jettison_dryrun_negation() {
        assert_eq!(
            CargoArgs::try_parse_from(["cargo", "liner", "jettison", "--dry-run", "--no-dry-run"])
                .unwrap(),
            CargoArgs::Liner(LinerArgs {
                command: Some(LinerCommands::Jettison(JettisonArgsWithNegations {
                    inner: JettisonArgs {
                        no_confirm: None,
                        no_fail_fast: None,
                        dry_run: Some(false),
                    },
                    _confirm: (),
                    _fail_fast: (),
                    _no_dry_run: (),
                })),
                verbose: 0,
                quiet: 0,
                color: ColorChoice::Auto,
            }),
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

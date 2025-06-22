//! Run arguments fetched from the environment.

use std::env::{self, VarError};
use std::error::Error;
use std::str::FromStr;

use color_eyre::Section;
use color_eyre::eyre::{Context, Result};

use crate::cli::{JettisonArgs, ShipArgs};

const SELF_ENV_PREFIX: &str = "CARGO_LINER";
const SHIP_ENV_PREFIX: &str = "SHIP";
const JETTISON_ENV_PREFIX: &str = "JETTISON";

/// Returns the full environment variable name from the given suffix to `ship`.
#[inline]
fn ship_var_name(suffix: &str) -> String {
    format!("{SELF_ENV_PREFIX}_{SHIP_ENV_PREFIX}_{suffix}")
}

/// Returns the full environment variable name from the given suffix to
/// `jettison`.
#[inline]
fn jettison_var_name(suffix: &str) -> String {
    format!("{SELF_ENV_PREFIX}_{JETTISON_ENV_PREFIX}_{suffix}")
}

/// Retrieves a single `ship` argument value from the environment.
///
/// Supports any destination type that implements string parsing.
fn get_ship_arg<T>(suffix: &str) -> Result<Option<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    get_arg(&ship_var_name(suffix))
}

/// Retrieves a single `jettison` argument value from the environment.
///
/// Supports any destination type that implements string parsing.
fn get_jettison_arg<T>(suffix: &str) -> Result<Option<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    get_arg(&jettison_var_name(suffix))
}

/// Retrieves a single argument value from the given full environment variable.
///
/// Supports any destination type that implements string parsing.
fn get_arg<T>(var_name: &str) -> Result<Option<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    match env::var(var_name) {
        Ok(val) => Ok(Some(
            val.parse()
                .wrap_err_with(|| {
                    format!(
                        "Could not parse an argument value from the {var_name:?} environment variable."
                    )
                })
                .note("Only the `true` or `false` values are accepted here, except for `binstall`.")
                .suggestion("Analyze your environment variables and correct the value.")?,
        )),
        Err(VarError::NotPresent) => Ok(None),
        Err(err) => Err(err)
            .wrap_err_with(|| {
                format!(
                    "Could not fetch a value from the {var_name:?} environment variable."
                )
            })
            .note("Only valid UTF-8 values are accepted here.")
            .suggestion("Analyze your environment variables and correct the value."),
    }
}

/// Returns the [`ShipArgs`] fetched from the environment.
pub fn ship_env_args() -> Result<ShipArgs> {
    Ok(ShipArgs {
        no_self: get_ship_arg("NO_SELF")?,
        only_self: get_ship_arg("ONLY_SELF")?,
        skip_check: get_ship_arg("SKIP_CHECK")?,
        no_fail_fast: get_ship_arg("NO_FAIL_FAST")?,
        force: get_ship_arg("FORCE")?,
        dry_run: get_ship_arg("DRY_RUN")?,
        binstall: get_ship_arg("BINSTALL")?,
    })
}

/// Returns the [`JettisonArgs`] fetched from the environment.
pub fn jettison_env_args() -> Result<JettisonArgs> {
    Ok(JettisonArgs {
        no_confirm: get_jettison_arg("NO_CONFIRM")?,
        no_fail_fast: get_jettison_arg("NO_FAIL_FAST")?,
        dry_run: get_jettison_arg("DRY_RUN")?,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::{LazyLock, Mutex};

    use super::*;
    use crate::cli::BinstallChoice;

    static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    fn set_vars<'a>(var_vals: impl IntoIterator<Item = &'a (&'static str, &'static str)>) {
        for (var, val) in var_vals {
            // SAFETY: all tests are mutually exclusive at runtime due to the lock.
            unsafe { env::set_var(var, val) };
        }
    }

    fn remove_vars<'a>(var_vals: impl IntoIterator<Item = &'a (&'static str, &'static str)>) {
        for (var, _val) in var_vals {
            // SAFETY: all tests are mutually exclusive at runtime due to the lock.
            unsafe { env::remove_var(var) };
        }
    }

    #[test]
    fn test_singlethreaded_ship_flags_ok() {
        let _lk = LOCK.lock().unwrap();
        let var_vals = [
            ("CARGO_LINER_SHIP_NO_SELF", "true"),
            ("CARGO_LINER_SHIP_FORCE", "false"),
            ("CARGO_LINER_SHIP_DRY_RUN", "true"),
        ];
        set_vars(&var_vals);

        assert_eq!(
            ship_env_args().unwrap(),
            ShipArgs {
                no_self: Some(true),
                force: Some(false),
                dry_run: Some(true),
                ..Default::default()
            }
        );

        remove_vars(&var_vals);
    }

    #[test]
    fn test_singlethreaded_ship_flags_errs() {
        let _lk = LOCK.lock().unwrap();

        for (var, val) in [
            ("CARGO_LINER_SHIP_ONLY_SELF", ""),
            ("CARGO_LINER_SHIP_NO_FAIL_FAST", " "),
            ("CARGO_LINER_SHIP_SKIP_CHECK", "123"),
            ("CARGO_LINER_SHIP_NO_SELF", "abc"),
            ("CARGO_LINER_SHIP_FORCE", "\x01"),
            ("CARGO_LINER_SHIP_DRY_RUN", "xyz"),
        ] {
            assert_eq!(ship_env_args().unwrap(), ShipArgs::default());
            set_vars(&[(var, val)]);
            assert!(ship_env_args().is_err());
            remove_vars(&[(var, val)]);
        }
    }

    #[test]
    fn test_singlethreaded_ship_binstall_ok() {
        let _lk = LOCK.lock().unwrap();

        for (var, val_str, val) in [
            ("CARGO_LINER_SHIP_BINSTALL", "auto", BinstallChoice::Auto),
            (
                "CARGO_LINER_SHIP_BINSTALL",
                "always",
                BinstallChoice::Always,
            ),
            ("CARGO_LINER_SHIP_BINSTALL", "never", BinstallChoice::Never),
        ] {
            assert_eq!(ship_env_args().unwrap(), ShipArgs::default());
            set_vars(&[(var, val_str)]);
            assert_eq!(ship_env_args().unwrap().binstall.unwrap(), val);
            remove_vars(&[(var, val_str)]);
        }
    }

    #[test]
    fn test_singlethreaded_ship_binstall_errs() {
        let _lk = LOCK.lock().unwrap();

        for (var, val) in [
            ("CARGO_LINER_SHIP_BINSTALL", ""),
            ("CARGO_LINER_SHIP_BINSTALL", " "),
            ("CARGO_LINER_SHIP_BINSTALL", "a"),
            ("CARGO_LINER_SHIP_BINSTALL", "abc"),
            ("CARGO_LINER_SHIP_BINSTALL", "123"),
            ("CARGO_LINER_SHIP_BINSTALL", "\x01"),
        ] {
            assert_eq!(ship_env_args().unwrap(), ShipArgs::default());
            set_vars(&[(var, val)]);
            assert!(ship_env_args().is_err());
            remove_vars(&[(var, val)]);
        }
    }

    #[test]
    fn test_singlethreaded_jettison_flags_ok() {
        let _lk = LOCK.lock().unwrap();
        let var_vals = [
            ("CARGO_LINER_JETTISON_NO_CONFIRM", "true"),
            ("CARGO_LINER_JETTISON_NO_FAIL_FAST", "false"),
        ];
        set_vars(&var_vals);

        assert_eq!(
            jettison_env_args().unwrap(),
            JettisonArgs {
                no_confirm: Some(true),
                no_fail_fast: Some(false),
                ..Default::default()
            }
        );

        remove_vars(&var_vals);
    }

    #[test]
    fn test_singlethreaded_jettison_flags_errs() {
        let _lk = LOCK.lock().unwrap();

        for (var, val) in [
            ("CARGO_LINER_JETTISON_NO_CONFIRM", ""),
            ("CARGO_LINER_JETTISON_NO_FAIL_FAST", " "),
            ("CARGO_LINER_JETTISON_DRY_RUN", "xyz"),
        ] {
            assert_eq!(jettison_env_args().unwrap(), JettisonArgs::default());
            set_vars(&[(var, val)]);
            assert!(jettison_env_args().is_err());
            remove_vars(&[(var, val)]);
        }
    }
}

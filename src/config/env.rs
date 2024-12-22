//! Run arguments fetched from the environment.

use std::env::{self, VarError};
use std::error::Error;
use std::str::FromStr;

use color_eyre::eyre::{Context, Result};
use color_eyre::Section;

use crate::cli::ShipArgs;

const SELF_ENV_PREFIX: &str = "CARGO_LINER";
const SHIP_ENV_PREFIX: &str = "SHIP";

#[inline]
fn ship_var_name(suffix: &str) -> String {
    format!("{SELF_ENV_PREFIX}_{SHIP_ENV_PREFIX}_{suffix}")
}

fn get_ship_flag<T>(suffix: &str) -> Result<Option<T>>
where
    T: FromStr,
    <T as FromStr>::Err: Error + Send + Sync + 'static,
{
    let var_name = ship_var_name(suffix);

    match env::var(&var_name) {
        Ok(val) => Ok(Some(
            val.parse()
                .wrap_err_with(|| {
                    format!(
                        "Could not parse an argument value from the {} environment variable.",
                        &var_name
                    )
                })
                .note("Only the `true` or `false` values are accepted here.")
                .suggestion("Analyze your environment variables and correct the value.")?,
        )),
        Err(VarError::NotPresent) => Ok(None),
        Err(err) => Err(err)
            .wrap_err_with(|| {
                format!(
                    "Could not fetch a value from the {} environment variable.",
                    &var_name
                )
            })
            .note("Only valid UTF-8 values are accepted here.")
            .suggestion("Analyze your environment variables and correct the value."),
    }
}

/// Returns the [`ShipArgs`] fetched from the environment.
pub fn ship_env_args() -> Result<ShipArgs> {
    Ok(ShipArgs {
        no_self: get_ship_flag("NO_SELF")?,
        only_self: get_ship_flag("ONLY_SELF")?,
        skip_check: get_ship_flag("SKIP_CHECK")?,
        no_fail_fast: get_ship_flag("NO_FAIL_FAST")?,
        force: get_ship_flag("FORCE")?,
        binstall: get_ship_flag("BINSTALL")?,
    })
}

#[cfg(test)]
mod tests {
    use std::sync::{LazyLock, Mutex};

    use super::*;

    static LOCK: LazyLock<Mutex<()>> = LazyLock::new(|| Mutex::new(()));

    fn set_vars<'a>(var_vals: impl IntoIterator<Item = &'a (&'static str, &'static str)>) {
        for (var, val) in var_vals {
            env::set_var(var, val);
        }
    }

    fn remove_vars<'a>(var_vals: impl IntoIterator<Item = &'a (&'static str, &'static str)>) {
        for (var, _val) in var_vals {
            env::remove_var(var);
        }
    }

    #[test]
    fn test_singlethreaded_ship_ok() {
        let _lk = LOCK.lock().unwrap();
        let var_vals = [
            ("CARGO_LINER_SHIP_NO_SELF", "true"),
            ("CARGO_LINER_SHIP_FORCE", "false"),
        ];
        set_vars(&var_vals);

        assert_eq!(
            ship_env_args().unwrap(),
            ShipArgs {
                no_self: Some(true),
                force: Some(false),
                ..Default::default()
            }
        );

        remove_vars(&var_vals);
    }

    #[test]
    fn test_singlethreaded_ship_errs() {
        let _lk = LOCK.lock().unwrap();

        for (var, val) in [
            ("CARGO_LINER_SHIP_ONLY_SELF", ""),
            ("CARGO_LINER_SHIP_NO_FAIL_FAST", " "),
            ("CARGO_LINER_SHIP_SKIP_CHECK", "123"),
            ("CARGO_LINER_SHIP_NO_SELF", "abc"),
            ("CARGO_LINER_SHIP_FORCE", "\x01"),
        ] {
            assert_eq!(ship_env_args().unwrap(), ShipArgs::default());
            set_vars(&[(var, val)]);
            assert!(ship_env_args().is_err());
            remove_vars(&[(var, val)]);
        }
    }
}

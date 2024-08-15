//! Run arguments fetched from the environment.

use std::env::{self, VarError};

use color_eyre::eyre::Result;

use crate::cli::ShipArgs;

const SELF_ENV_PREFIX: &str = "CARGO_LINER";
const SHIP_ENV_PREFIX: &str = "SHIP";

#[inline]
fn ship_var_name(suffix: &str) -> String {
    format!("{SELF_ENV_PREFIX}_{SHIP_ENV_PREFIX}_{suffix}")
}

fn get_ship_flag(suffix: &str) -> Result<Option<bool>> {
    match env::var(ship_var_name(suffix)) {
        Ok(val) => Ok(Some(val.parse()?)),
        Err(VarError::NotPresent) => Ok(None),
        Err(err) => Err(err.into()),
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
    })
}

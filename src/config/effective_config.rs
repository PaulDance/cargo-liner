//! Mainly exports [`EffectiveConfig`]: a reunion of all configuration sources.
//!
//! This is a bit the role of crates such as `twelf`, simply re-done here to be
//! customized for the needs specific to the current project, which includes
//! parsing the various sources in split steps, doing it optionally, and also
//! sometimes only partially.

use std::collections::BTreeMap;

use super::{DetailedPackageReq, UserConfig};
use crate::cli::{BinstallChoice, ShipArgs};

/// Effective merge of all configuration sources.
#[derive(Debug)]
pub struct EffectiveConfig {
    /// The name-to-setting map derived from the [`UserConfig`].
    pub packages: BTreeMap<String, DetailedPackageReq>,
    /// Effective arguments to use for the `ship` CLI command.
    pub ship_args: EffectiveShipArgs,
}

impl EffectiveConfig {
    /// Merges all given sources and exports the result as public fields.
    pub fn new(user_config: UserConfig, env_args: ShipArgs, cli_args: ShipArgs) -> Self {
        let ship_args = EffectiveShipArgs::new(&user_config, env_args, cli_args);
        Self {
            packages: user_config
                .self_update(!ship_args.no_self)
                .update_others(!ship_args.only_self)
                .packages
                .into_iter()
                .map(|(pkg_name, pkg)| (pkg_name, pkg.into()))
                .collect::<BTreeMap<String, DetailedPackageReq>>(),
            ship_args,
        }
    }
}

/// Effective merge of all configuration sources regarding [`ShipArgs`].
#[derive(Debug)]
#[cfg_attr(test, derive(Default, PartialEq, Eq))]
#[expect(
    clippy::struct_excessive_bools,
    reason = "This includes all the `ship` CLI boolean flags, which are not in small number."
)]
pub struct EffectiveShipArgs {
    pub no_self: bool,
    pub only_self: bool,
    pub skip_check: bool,
    pub no_fail_fast: bool,
    pub force: bool,
    pub binstall: BinstallChoice,
}

impl EffectiveShipArgs {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "Ensures the original arguments cannot be used by mistake anymore by consuming them."
    )]
    fn new(user_config: &UserConfig, env_args: ShipArgs, cli_args: ShipArgs) -> Self {
        let cfg_defs = user_config.defaults.as_ref();
        Self {
            no_self: cli_args
                .no_self
                .or(env_args.no_self)
                .or_else(|| cfg_defs.and_then(|defs| defs.ship_cmd.no_self.as_ref().copied()))
                .unwrap_or_default(),
            only_self: cli_args
                .only_self
                .or(env_args.only_self)
                .or_else(|| cfg_defs.and_then(|defs| defs.ship_cmd.only_self.as_ref().copied()))
                .unwrap_or_default(),
            skip_check: cli_args
                .skip_check
                .or(env_args.skip_check)
                .or_else(|| cfg_defs.and_then(|defs| defs.ship_cmd.skip_check.as_ref().copied()))
                .unwrap_or_default(),
            no_fail_fast: cli_args
                .no_fail_fast
                .or(env_args.no_fail_fast)
                .or_else(|| cfg_defs.and_then(|defs| defs.ship_cmd.no_fail_fast.as_ref().copied()))
                .unwrap_or_default(),
            force: cli_args
                .force
                .or(env_args.force)
                .or_else(|| cfg_defs.and_then(|defs| defs.ship_cmd.force.as_ref().copied()))
                .unwrap_or_default(),
            binstall: cli_args
                .binstall
                .or(env_args.binstall)
                .or_else(|| cfg_defs.and_then(|defs| defs.ship_cmd.binstall.as_ref().copied()))
                .unwrap_or_default(),
        }
    }
}

#[cfg(test)]
mod tests {
    #![expect(
        clippy::needless_update,
        reason = "Forward-compatibility and symmetry."
    )]
    use std::fmt::Debug;

    use super::*;
    use crate::config::user_config::DefaultsSection;

    /// Refactor safe-guard: check that both [`crate::cli::ShipArgs`] and
    /// [`EffectiveShipArgs`] have the exact same fields.
    #[test]
    fn test_effectiveshipargs_haswholecli() {
        // HACK: exploit the `Debug` implementation in order to get the field
        // names. Use `Default` in order to get a value to feed into `Debug`.
        fn debug_struct_fields<T: Debug + Default>() -> Vec<String> {
            format!("{:#?}", T::default())
                .lines()
                .skip(1)
                .filter(|line| line.starts_with("    "))
                .map(|line| line.trim_start().split(':').next().unwrap().to_owned())
                .collect()
        }

        assert_eq!(
            debug_struct_fields::<crate::cli::ShipArgs>(),
            debug_struct_fields::<EffectiveShipArgs>()
        );
    }

    #[test]
    fn test_effectiveshipargs_allnone_isdefault() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: None,
                },
                ShipArgs::default(),
                ShipArgs::default(),
            ),
            EffectiveShipArgs::default(),
        );
    }

    #[test]
    fn test_effectiveshipargs_configempty_isdefault() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: Some(DefaultsSection {
                        ship_cmd: ShipArgs::default()
                    }),
                },
                ShipArgs::default(),
                ShipArgs::default(),
            ),
            EffectiveShipArgs::default(),
        );
    }

    #[test]
    fn test_effectiveshipargs_configset_noenv_noargs_isconfig() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: Some(DefaultsSection {
                        ship_cmd: ShipArgs {
                            force: Some(false),
                            no_self: Some(true),
                            binstall: Some(BinstallChoice::Always),
                            ..Default::default()
                        }
                    }),
                },
                ShipArgs::default(),
                ShipArgs::default(),
            ),
            EffectiveShipArgs {
                force: false,
                no_self: true,
                binstall: BinstallChoice::Always,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_effectiveshipargs_noconfig_envset_noargs_isenv() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: Some(DefaultsSection {
                        ship_cmd: ShipArgs::default()
                    }),
                },
                ShipArgs {
                    force: Some(false),
                    no_self: Some(true),
                    binstall: Some(BinstallChoice::Auto),
                    ..Default::default()
                },
                ShipArgs::default(),
            ),
            EffectiveShipArgs {
                force: false,
                no_self: true,
                binstall: BinstallChoice::Auto,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_effectiveshipargs_noconfig_noenv_argsset_isargs() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: Some(DefaultsSection {
                        ship_cmd: ShipArgs::default()
                    }),
                },
                ShipArgs::default(),
                ShipArgs {
                    no_fail_fast: Some(true),
                    skip_check: Some(false),
                    binstall: Some(BinstallChoice::Never),
                    ..Default::default()
                },
            ),
            EffectiveShipArgs {
                no_fail_fast: true,
                skip_check: false,
                binstall: BinstallChoice::Never,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_effectiveshipargs_configset_envset_argsset_nointersection_isunion() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: Some(DefaultsSection {
                        ship_cmd: ShipArgs {
                            force: Some(true),
                            no_self: Some(false),
                            ..Default::default()
                        }
                    }),
                },
                ShipArgs {
                    only_self: Some(true),
                    binstall: Some(BinstallChoice::Always),
                    ..Default::default()
                },
                ShipArgs {
                    no_fail_fast: Some(true),
                    skip_check: Some(false),
                    ..Default::default()
                },
            ),
            EffectiveShipArgs {
                force: true,
                no_self: false,
                no_fail_fast: true,
                skip_check: false,
                only_self: true,
                binstall: BinstallChoice::Always,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_effectiveshipargs_configset_envset_noargs_withintersection_envhasprecedence() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: Some(DefaultsSection {
                        ship_cmd: ShipArgs {
                            force: Some(true),
                            no_self: Some(false),
                            no_fail_fast: Some(true),
                            skip_check: Some(false),
                            binstall: Some(BinstallChoice::Always),
                            ..Default::default()
                        }
                    }),
                },
                ShipArgs {
                    force: Some(false),
                    no_self: Some(true),
                    no_fail_fast: Some(true),
                    skip_check: Some(false),
                    binstall: Some(BinstallChoice::Auto),
                    ..Default::default()
                },
                ShipArgs::default(),
            ),
            EffectiveShipArgs {
                force: false,
                no_self: true,
                no_fail_fast: true,
                skip_check: false,
                binstall: BinstallChoice::Auto,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_effectiveshipargs_noconfig_envset_argsset_withintersection_argshaveprecedence() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: None,
                },
                ShipArgs {
                    force: Some(true),
                    no_self: Some(false),
                    no_fail_fast: Some(true),
                    skip_check: Some(false),
                    binstall: Some(BinstallChoice::Always),
                    ..Default::default()
                },
                ShipArgs {
                    force: Some(false),
                    no_self: Some(true),
                    no_fail_fast: Some(true),
                    skip_check: Some(false),
                    binstall: Some(BinstallChoice::Never),
                    ..Default::default()
                },
            ),
            EffectiveShipArgs {
                force: false,
                no_self: true,
                no_fail_fast: true,
                skip_check: false,
                binstall: BinstallChoice::Never,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_effectiveshipargs_configset_noenv_argsset_withintersection_argshaveprecedence() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: Some(DefaultsSection {
                        ship_cmd: ShipArgs {
                            force: Some(true),
                            no_self: Some(false),
                            no_fail_fast: Some(true),
                            skip_check: Some(false),
                            binstall: Some(BinstallChoice::Auto),
                            ..Default::default()
                        }
                    }),
                },
                ShipArgs::default(),
                ShipArgs {
                    force: Some(false),
                    no_self: Some(true),
                    no_fail_fast: Some(true),
                    skip_check: Some(false),
                    binstall: Some(BinstallChoice::Never),
                    ..Default::default()
                },
            ),
            EffectiveShipArgs {
                force: false,
                no_self: true,
                no_fail_fast: true,
                skip_check: false,
                binstall: BinstallChoice::Never,
                ..Default::default()
            },
        );
    }

    #[test]
    fn test_effectiveshipargs_configset_envset_argsset_withintersection_argshaveprecedence() {
        assert_eq!(
            EffectiveShipArgs::new(
                &UserConfig {
                    packages: BTreeMap::new(),
                    defaults: Some(DefaultsSection {
                        ship_cmd: ShipArgs {
                            force: Some(true),
                            no_self: Some(false),
                            no_fail_fast: Some(true),
                            skip_check: Some(false),
                            binstall: Some(BinstallChoice::Never),
                            ..Default::default()
                        }
                    }),
                },
                ShipArgs {
                    only_self: Some(false),
                    binstall: Some(BinstallChoice::Auto),
                    ..Default::default()
                },
                ShipArgs {
                    force: Some(false),
                    no_self: Some(true),
                    no_fail_fast: Some(true),
                    skip_check: Some(false),
                    only_self: Some(true),
                    binstall: Some(BinstallChoice::Always),
                    ..Default::default()
                },
            ),
            EffectiveShipArgs {
                force: false,
                no_self: true,
                no_fail_fast: true,
                skip_check: false,
                only_self: true,
                binstall: BinstallChoice::Always,
                ..Default::default()
            },
        );
    }
}

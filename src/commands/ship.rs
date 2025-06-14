use std::collections::{BTreeMap, BTreeSet};

use color_eyre::Result;
use color_eyre::eyre::Context;
use semver::Version;
use tabled::settings::Style;
use tabled::{Table, Tabled};

use crate::cargo::{self, InstallStatus};
use crate::coloring::Colorizer;
use crate::config::{CargoCratesToml, DetailedPackageReq, EffectiveConfig};

pub fn run(config: &EffectiveConfig, colorizer: &Colorizer, cargo_verbosity: i8) -> Result<()> {
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
                *colorizer.color(),
                cargo_verbosity,
            ),
            BTreeMap::new(),
            BTreeMap::new(),
        )
    } else {
        let cct =
            CargoCratesToml::parse_file().wrap_err("Failed to parse Cargo's .crates.toml file.")?;
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
        log_version_check_summary(colorizer, &config.packages, &new_vers, &old_vers);

        (
            cargo::install_all(
                &needing_install(&config.packages, &new_vers, &old_vers),
                &cct.into_names(),
                config.ship_args.no_fail_fast,
                config.ship_args.force,
                config.ship_args.dry_run,
                config.ship_args.binstall,
                *colorizer.color(),
                cargo_verbosity,
            ),
            old_vers,
            new_vers,
        )
    };

    if let Some(err) = match inst_res {
        Ok(rep) => {
            log_install_report(
                colorizer,
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

    Ok(())
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

/// [`Tabled`] for logging package statuses.
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

use color_eyre::eyre::Context;
use color_eyre::{Result, Section, eyre};

use crate::cli::ImportArgs;
use crate::config::{CargoCratesToml, UserConfig};

pub fn run(args: &ImportArgs) -> Result<()> {
    if UserConfig::file_path()
        .wrap_err("Failed to build the configuration file path.")?
        .try_exists()
        .wrap_err("Failed to check if the configuration file exists.")
        .suggestion("Check the permissions of the Cargo home directory.")?
    {
        if args.force {
            log::warn!("Configuration file will be overwritten.");
        } else {
            eyre::bail!("Configuration file already exists, use -f/--force to overwrite.");
        }
    }

    log::info!("Importing Cargo installed crates as a new configuration file...");

    // Clap conflict settings ensure the options are mutually exclusive.
    (if args.force {
        UserConfig::overwrite_file
    } else {
        UserConfig::save_file
    })(&(if args.exact {
        CargoCratesToml::into_exact_version_config
    } else if args.compatible {
        CargoCratesToml::into_comp_version_config
    } else if args.patch {
        CargoCratesToml::into_patch_version_config
    } else {
        CargoCratesToml::into_star_version_config
    })(
        CargoCratesToml::parse_file().wrap_err("Failed to parse Cargo's .crates.toml file.")?,
        args.keep_self,
        args.keep_local,
    ))
    .wrap_err("Failed to save the configuration file.")
}

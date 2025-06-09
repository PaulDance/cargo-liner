use std::io;

use clap::CommandFactory;

use crate::cli::{CompletionsArgs, LinerArgs};

pub fn run(args: &CompletionsArgs) {
    log::info!("Generating auto-completion script for {:?}...", args.shell);
    clap_complete::generate(
        args.shell,
        &mut LinerArgs::command(),
        clap::crate_name!(),
        &mut io::stdout(),
    );
}

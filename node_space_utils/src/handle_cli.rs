use crate::cli_manager::{Cli, Commands};
use crate::errors::node_space::NodeSpaceError;
use crate::link_package::handle_link_command;

use clap::Parser;

pub fn handle_cli() -> Result<bool, NodeSpaceError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Link(link_args) => handle_link_command(link_args),
    }
}

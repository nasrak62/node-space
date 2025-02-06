use clap::{Parser, Subcommand};

use crate::args::group_args::{GroupShowArgs, GroupsArgs};

#[derive(Subcommand)]
pub enum GroupCommands {
    Add(GroupsArgs),
    Show(GroupShowArgs),
}

#[derive(Parser)]
pub struct GroupCommandBridge {
    #[structopt(subcommand)]
    pub group_commands: GroupCommands,
}

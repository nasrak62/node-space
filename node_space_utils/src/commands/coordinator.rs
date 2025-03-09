use clap::{Parser, Subcommand};

use crate::args::coordinator_args::{CoordinatorLogArgs, CoordinatorStartArgs};

#[derive(Subcommand)]
pub enum CoordinatorCommands {
    Start(CoordinatorStartArgs),
    Log(CoordinatorLogArgs),
}

#[derive(Parser)]
pub struct CoordinatorBridge {
    #[structopt(subcommand)]
    pub coordinator_commands: CoordinatorCommands,
}

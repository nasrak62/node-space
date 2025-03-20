use clap::{Parser, Subcommand};

use crate::args::server_args::{ConfigServerArgs, StartServerArgs};

#[derive(Subcommand)]
pub enum ServerCommands {
    Start(StartServerArgs),
    Config(ConfigServerArgs),
}

#[derive(Parser)]
pub struct ServerBridge {
    #[structopt(subcommand)]
    pub server_commands: ServerCommands,
}

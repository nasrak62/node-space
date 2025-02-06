use clap::{Parser, Subcommand};

use crate::args::project_args::ProjectArgs;

#[derive(Subcommand)]
pub enum ProjectCommands {
    Add(ProjectArgs),
    Show,
}

#[derive(Parser)]
pub struct ProjectCommandBridge {
    #[structopt(subcommand)]
    pub project_commands: ProjectCommands,
}

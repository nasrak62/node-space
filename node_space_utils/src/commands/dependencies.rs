use clap::{Parser, Subcommand};

use crate::args::dependencies_args::{AddDependencyArgs, UpdateDependencyArgs};

#[derive(Subcommand)]
pub enum DependenciesCommands {
    Add(AddDependencyArgs),
    Update(UpdateDependencyArgs),
}

#[derive(Parser)]
pub struct DependenciesBridge {
    #[structopt(subcommand)]
    pub dependencies_commands: DependenciesCommands,
}

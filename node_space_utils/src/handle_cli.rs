use crate::build_command::build_project::handle_build_command;
use crate::cli_manager::Cli;
use crate::commands::base_command::Commands;
use crate::commands::dependencies::DependenciesCommands;
use crate::commands::group::GroupCommands;
use crate::commands::project::ProjectCommands;
use crate::dependencies::add::add_dependency;
use crate::dependencies::update::update_dependency;
use crate::errors::node_space::NodeSpaceError;
use crate::groups::add::add_group;
use crate::groups::show::show_group;
use crate::link_package::handle_link_command;
use crate::projects::add::add_project;
use crate::projects::show::show_all_projects;

use clap::Parser;

pub fn handle_cli() -> Result<bool, NodeSpaceError> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Link(link_args) => handle_link_command(link_args),
        Commands::Project(project_command) => match &project_command.project_commands {
            ProjectCommands::Add(project_args) => add_project(&project_args),
            ProjectCommands::Show => show_all_projects(),
        },

        Commands::Group(group_command) => match &group_command.group_commands {
            GroupCommands::Add(group_args) => add_group(&group_args),
            GroupCommands::Show(group_show_args) => show_group(&group_show_args),
        },
        Commands::Deps(dependency_command) => match &dependency_command.dependencies_commands {
            DependenciesCommands::Add(add_dependency_args) => add_dependency(&add_dependency_args),
            DependenciesCommands::Update(update_dependency_args) => {
                update_dependency(&update_dependency_args)
            }
        },
        Commands::Build(build_args) => handle_build_command(build_args),
    }
}

use clap::Subcommand;

use crate::args::build_args::BuildArgs;
use crate::args::link_args::LinkArgs;

use super::dependencies::DependenciesBridge;
use super::group::GroupCommandBridge;
use super::project::ProjectCommandBridge;

#[derive(Subcommand)]
pub enum Commands {
    Link(LinkArgs),
    Project(ProjectCommandBridge),
    Group(GroupCommandBridge),
    Deps(DependenciesBridge),
    Build(BuildArgs),
}

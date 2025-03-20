use clap::Subcommand;

use crate::args::build_args::BuildArgs;
use crate::args::link_args::LinkArgs;

use super::coordinator::CoordinatorBridge;
use super::dependencies::DependenciesBridge;
use super::group::GroupCommandBridge;
use super::project::ProjectCommandBridge;
use super::server::ServerBridge;

#[derive(Subcommand)]
pub enum Commands {
    Link(LinkArgs),
    Project(ProjectCommandBridge),
    Group(GroupCommandBridge),
    Deps(DependenciesBridge),
    Build(BuildArgs),
    Coordinator(CoordinatorBridge),
    Server(ServerBridge),
}

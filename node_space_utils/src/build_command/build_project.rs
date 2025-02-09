use crate::{
    args::build_args::BuildArgs,
    command_line::node_build::run_node_command,
    errors::node_space::NodeSpaceError,
    modals::{config_file::ConfigFile, package::Package, socket_build_data::SocketBuildData},
    package_utils::get_base_package_data,
    path_utils::get_current_path,
    watch_coordinator::client::start_coordinator::{
        add_local_project_watcher, request_build_watcher_for_project, start_coordinator,
    },
};

pub fn handle_watch_project_with_dependencies(
    data: SocketBuildData,
) -> Result<bool, NodeSpaceError> {
    start_coordinator()?;
    request_build_watcher_for_project()?;

    if data.watch_only_links {
        add_local_project_watcher(data.project)?;
    }

    Ok(true)
}

pub fn handle_build_command(args: &BuildArgs) -> Result<bool, NodeSpaceError> {
    if !args.watch && !args.start {
        let path = get_current_path()?;

        return Ok(run_node_command(&path, "build")?);
    }

    if !args.watch && args.start {
        let path = get_current_path()?;

        return Ok(run_node_command(&path, "start")?);
    }

    let config_file = ConfigFile::new()?;
    let (_, package_name, current_path) = get_base_package_data(None)?;
    let current_project = Package::new(current_path, package_name.clone(), None);

    let current_symlink_option = config_file.symlinks.get(&package_name);
    let has_symlinks = current_symlink_option.is_some();
    let is_local_watcher = !args.deamon;

    if !has_symlinks && is_local_watcher {
        add_local_project_watcher(current_project)?;

        return Ok(true);
    }

    let socket_data = SocketBuildData::new(
        current_symlink_option.unwrap().to_vec(),
        current_project,
        !args.deamon,
    );

    handle_watch_project_with_dependencies(socket_data)
}

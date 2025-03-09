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
    dbg!("handle_watch_project_with_dependencies");

    start_coordinator()?;
    request_build_watcher_for_project(data.clone())?;

    if data.watch_only_links {
        dbg!("watch_only_links");

        add_local_project_watcher(data)?;

        return Ok(true);
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
    let is_local_watcher = !args.deamon;

    let effective_symlinks = match current_symlink_option {
        Some(value) => value.to_vec(),
        None => Vec::new(),
    };

    let has_symlinks = !effective_symlinks.is_empty();

    dbg!("{}", &effective_symlinks);

    let socket_data = SocketBuildData::new(effective_symlinks, current_project, is_local_watcher);

    if !has_symlinks && is_local_watcher {
        dbg!("only local watcher");

        add_local_project_watcher(socket_data)?;

        return Ok(true);
    }

    handle_watch_project_with_dependencies(socket_data)
}

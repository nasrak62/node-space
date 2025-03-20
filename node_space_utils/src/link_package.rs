use crate::{
    args::link_args::LinkArgs,
    display_utils::{display_symlink_graph, handle_show_linked_packages},
    errors::node_space::NodeSpaceError,
    modals::config_file::ConfigFile,
    package_utils::get_base_package_data,
};

pub fn link_package(
    package_path: Option<&str>,
    package_name_alias: Option<String>,
    output_dir: Option<String>,
) -> Result<bool, NodeSpaceError> {
    let mut config_file = ConfigFile::new()?;

    let (_, package_name, current_path) = get_base_package_data(package_path)?;

    config_file.add_linked_package(current_path, &package_name, package_name_alias, output_dir)?;

    Ok(true)
}

pub fn handle_link_command(link_args: &LinkArgs) -> Result<bool, NodeSpaceError> {
    if link_args.show && link_args.graph {
        return Ok(display_symlink_graph()?);
    }

    if link_args.show {
        return Ok(handle_show_linked_packages()?);
    }

    let alias = &link_args.name;

    Ok(link_package(
        None,
        alias.clone(),
        link_args.output_dir.clone(),
    )?)
}

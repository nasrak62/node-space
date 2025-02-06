use crate::{
    args::project_args::ProjectArgs,
    errors::node_space::NodeSpaceError,
    modals::{config_file::ConfigFile, package::Package},
    package_utils::get_base_package_data,
};

pub fn add_project(_: &ProjectArgs) -> Result<bool, NodeSpaceError> {
    let mut config_file = ConfigFile::new()?;

    let (_, package_name, current_path) = get_base_package_data(None)?;
    let package = Package::new(current_path, package_name, None);

    config_file.add_project(&package)?;

    Ok(true)
}

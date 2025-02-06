use crate::modals::package::Package;
use crate::{args::group_args::GroupsArgs, errors::node_space::NodeSpaceError};

use crate::{modals::config_file::ConfigFile, package_utils::get_base_package_data};

pub fn add_group(group_args: &GroupsArgs) -> Result<bool, NodeSpaceError> {
    let mut config_file = ConfigFile::new()?;

    let (_, package_name, current_path) = get_base_package_data(None)?;
    let package = Package::new(current_path, package_name, None);

    config_file.add_group(&package, &group_args.name)?;

    Ok(true)
}

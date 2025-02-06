use crate::{
    args::group_args::GroupShowArgs,
    errors::{config_file::ConfigFileError, node_space::NodeSpaceError},
    modals::config_file::ConfigFile,
};

pub fn show_group(group_show_args: &GroupShowArgs) -> Result<bool, NodeSpaceError> {
    let config_file = ConfigFile::new()?;

    if group_show_args.name.is_none() {
        for (group, projects) in config_file.groups {
            println!("📦 {}", group);

            for package in projects {
                println!(" ├── 📁  name: {}, at ({})", package.name, package.path);
            }

            println!(); // New line for spacing
        }

        return Ok(true);
    }

    let name = group_show_args.name.clone().unwrap();

    let current_group = config_file.groups.get(&name);

    if current_group.is_none() {
        return Err(NodeSpaceError::ConfigFileError(
            ConfigFileError::InvalidGroupName,
        ));
    }

    for project in current_group.unwrap().iter() {
        println!("⚽ name: {}, at ({})", project.name, project.path);
    }

    Ok(true)
}

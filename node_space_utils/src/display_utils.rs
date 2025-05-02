use crate::{errors::node_space::NodeSpaceError, modals::config_file::ConfigFile};

pub fn handle_show_linked_packages() -> Result<bool, NodeSpaceError> {
    let config_file = ConfigFile::new(None)?;

    for value in config_file.linked_packages.iter() {
        let alias = match &value.alias {
            Some(value) => value,
            None => "",
        };

        println!(
            "⚽ name: {}, alias: {}, at ({})",
            value.name, alias, value.path
        );
    }

    Ok(true)
}

pub fn display_symlink_graph() -> Result<bool, NodeSpaceError> {
    let config_file = ConfigFile::new(None)?;

    for (project, linked_packages) in config_file.symlinks {
        println!("📦 {}", project);

        for package in linked_packages {
            let alias = match &package.alias {
                Some(value) => value,
                None => "",
            };

            println!(
                " ├── 📁  name: {}, alias: {}, at ({})",
                package.name, alias, package.path
            );
        }

        println!(); // New line for spacing
    }

    Ok(true)
}

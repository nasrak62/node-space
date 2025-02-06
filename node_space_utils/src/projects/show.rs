use crate::{errors::node_space::NodeSpaceError, modals::config_file::ConfigFile};

pub fn show_all_projects() -> Result<bool, NodeSpaceError> {
    let config_file = ConfigFile::new()?;

    for project in config_file.projects.iter() {
        println!("⚽ name: {}, at ({})", project.name, project.path);
    }

    Ok(true)
}

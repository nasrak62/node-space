use crate::errors::node_space::NodeSpaceError;
use crate::modals::config_file::ConfigFile;

pub fn sync_links() -> Result<bool, NodeSpaceError> {
    let mut config_file = ConfigFile::new(None)?;

    config_file.sync_links()
}

#[cfg(test)]
mod test;

use std::{fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::{
    errors::{config_file::ConfigFileError, invalid_project::InvalidNodeProjectError},
    path_utils::expand_tilde,
};

use super::package::Package;

const CONFIG_PATH_STR: &str = "~/.config/node-space/space-data.json";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConfigFile {
    pub linked_pachages: Vec<Package>,
    config_path: PathBuf,
}

impl ConfigFile {
    pub fn new() -> Result<Self, InvalidNodeProjectError> {
        let config_path = expand_tilde(CONFIG_PATH_STR)?;

        let json_data = fs::read_to_string(&config_path).unwrap_or_else(|_| String::from("{}"));

        let mut config: ConfigFile = serde_json::from_str(&json_data).unwrap_or_default();

        config.config_path = config_path;

        Ok(config)
    }

    pub fn save(&self) -> Result<(), ConfigFileError> {
        let json_data = match serde_json::to_string_pretty(self) {
            Ok(value) => value,
            Err(_) => return Err(ConfigFileError::CantDesirialize),
        };

        let parent_dir = match self.config_path.parent() {
            Some(value) => value,
            None => return Err(ConfigFileError::CantCreateDir),
        };

        match fs::create_dir_all(parent_dir) {
            Ok(value) => value,
            Err(_) => return Err(ConfigFileError::CantCreateDir),
        };

        match fs::write(&self.config_path, &json_data) {
            Ok(_) => Ok(()),
            Err(error) => Err(ConfigFileError::CantSerialize(error.to_string())),
        }?;

        Ok(())
    }

    pub fn add_linked_package(
        &mut self,
        new_path: String,
        package_name: &str,
    ) -> Result<(), ConfigFileError> {
        for package in self.linked_pachages.iter() {
            if package.path == new_path {
                return Ok(());
            }
        }

        self.linked_pachages
            .push(Package::new(new_path, package_name.to_string()));
        let result = self.save()?;

        Ok(result)
    }
}

use std::{collections::HashMap, fs, path::PathBuf};

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
    pub symlinks: HashMap<String, Vec<Package>>,
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

    pub fn create_symlink(
        &mut self,
        new_path: String,
        package_name: &str,
    ) -> Result<(), ConfigFileError> {
        if (!self.symlinks[package_name]) {}

        self.symlinks[package_name];
        self.linked_pachages
            .push(Package::new(new_path, package_name.to_string()));

        let result = self.save()?;

        Ok(result)
    }

    pub fn handle_link(
        &mut self,
        new_path: String,
        package_name: &str,
        package_alias: Option<String>,
    ) -> Result<(), ConfigFileError> {
        self.linked_pachages.push(Package::new(
            new_path,
            package_name.to_string(),
            package_alias,
        ));

        let result = self.save()?;

        Ok(result)
    }

    pub fn add_linked_package(
        &mut self,
        new_path: String,
        package_name: &str,
        package_alias: Option<String>,
    ) -> Result<(), ConfigFileError> {
        let mut is_symlink = false;

        for package in self.linked_pachages.iter() {
            let is_same_path = package.path == new_path;
            let is_same_name = package.name == package_name;

            let is_match_alias = match package_alias {
                Some(ref value) => value == package_name,
                None => false,
            };

            let is_alias_refrencing = match package_alias {
                Some(ref value) => match package_alias {
                    Some(ref out_value) => value == out_value,
                    None => false,
                },
                None => false,
            };

            let is_same_target = is_same_name || is_match_alias || is_alias_refrencing;

            if is_same_path {
                return Ok(());
            }

            if !is_same_path && is_same_target {
                is_symlink = true;

                break;
            }
        }

        if is_symlink {
            return Ok(self.create_symlink(new_path, package_name)?);
        }

        Ok(self.handle_link(new_path, package_name, package_alias)?)
    }
}

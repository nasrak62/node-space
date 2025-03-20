use std::os::unix::fs::symlink;
use std::{collections::HashMap, fs, path::PathBuf};

use serde::{Deserialize, Serialize};

use crate::errors::node_space::NodeSpaceError;
use crate::errors::symlink::SymlinkError;
use crate::package_utils::{find_package_by_name, is_package_exist};
use crate::path_utils::get_package_path_from_node_modules;
use crate::symlink_utils::handle_link_candidate;
use crate::{
    errors::{config_file::ConfigFileError, invalid_project::InvalidNodeProjectError},
    path_utils::expand_tilde,
};

use super::link_action::LinkAction;
use super::package::Package;
use super::server_config::ServerConfig;

const CONFIG_PATH_STR: &str = "~/.config/node-space/space-data.json";

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConfigFile {
    pub linked_packages: Vec<Package>,
    pub projects: Vec<Package>,
    pub symlinks: HashMap<String, Vec<Package>>,
    pub groups: HashMap<String, Vec<Package>>,
    pub server_config: HashMap<String, ServerConfig>,
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

    pub fn create_symlink(&mut self, current_package: &Package) -> Result<(), NodeSpaceError> {
        let link_to_name = match current_package.alias {
            Some(ref value) => value,
            None => {
                return Err(NodeSpaceError::SymlinkError(
                    SymlinkError::MissingLinkToTargetName,
                ))
            }
        };

        if self.symlinks.get(&current_package.name).is_none() {
            self.symlinks
                .insert(current_package.name.to_string(), Vec::new());
        }

        if !is_package_exist(&self.projects, &current_package.path) {
            self.projects.push(current_package.clone())
        }

        let package = find_package_by_name(&self.linked_packages, link_to_name)?;

        let list = self.symlinks.get_mut(&current_package.name).unwrap();

        list.push(package.clone());

        let symlink_path =
            get_package_path_from_node_modules(&current_package.path, &package.name)?;

        dbg!(&symlink_path, &package.path);

        match symlink(&package.path, symlink_path) {
            Ok(_) => {}
            Err(error) => {
                return Err(NodeSpaceError::ConfigFileError(
                    ConfigFileError::FailedToCreateSymLink(error.to_string()),
                ))
            }
        };

        let result = self.save()?;

        Ok(result)
    }

    pub fn handle_link(&mut self, current_package: &Package) -> Result<(), ConfigFileError> {
        self.linked_packages.push(current_package.clone());

        if !is_package_exist(&self.projects, &current_package.path) {
            self.projects.push(current_package.clone())
        }

        let result = self.save()?;

        Ok(result)
    }

    pub fn add_project(&mut self, current_package: &Package) -> Result<(), NodeSpaceError> {
        if !is_package_exist(&self.projects, &current_package.path) {
            self.projects.push(current_package.clone())
        }

        let result = self.save()?;

        Ok(result)
    }

    pub fn add_group(
        &mut self,
        current_package: &Package,
        group_name: &str,
    ) -> Result<(), NodeSpaceError> {
        if self.groups.get(group_name).is_none() {
            self.groups.insert(group_name.to_string(), Vec::new());
        }

        let list = self.groups.get_mut(group_name).unwrap();

        if !is_package_exist(&list, &current_package.path) {
            list.push(current_package.clone());
        }

        let result = self.save()?;

        Ok(result)
    }

    pub fn add_linked_package(
        &mut self,
        new_path: String,
        package_name: &str,
        package_alias: Option<String>,
        output_dir: Option<String>,
    ) -> Result<(), NodeSpaceError> {
        // case 1: package1 . -> link package1 with name package1
        // case 2: package1 test -> link package1 with alias test
        // case 3: package1 again -> do nothing (alias and named)
        // case 4: package2 link to test -> link package2 with test
        // case 5: package2 . -> link package2 with name package2
        // case 6: package2 test2 -> link package2 with name test2
        // states -> do nothing, link self, link to another package
        let current_package = Package::new(
            new_path,
            package_name.to_string(),
            package_alias,
            output_dir,
        );
        let action = handle_link_candidate(&self.linked_packages, &current_package);

        match action {
            LinkAction::LinkSelf => Ok(self.handle_link(&current_package)?),
            LinkAction::DoNothing => Ok(()),
            LinkAction::LinkToAnother => match self.create_symlink(&current_package) {
                Ok(_) => Ok(()),
                Err(error) => Err(error),
            },
        }
    }

    pub fn find_package(&self, path: String) -> Result<Package, NodeSpaceError> {
        let projects = self.projects.clone();

        let package = projects.iter().find(|x| x.path == path);

        if package.is_none() {
            return Err(NodeSpaceError::MissingProject);
        }

        let package = package.unwrap();

        Ok(package.clone())
    }

    pub fn build_name_project_mapper(&self) -> HashMap<String, Package> {
        let mut map: HashMap<String, Package> = HashMap::new();
        let projects = self.projects.clone();

        for project in projects {
            let keys = [Some(project.name.clone()), project.alias.clone()];

            for key in keys {
                if key.is_none() {
                    continue;
                }

                map.entry(key.unwrap())
                    .and_modify(|e| *e = project.clone())
                    .or_insert(project.clone());
            }
        }

        map
    }
}

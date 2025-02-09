use crate::{
    errors::{config_file::ConfigFileError, node_space::NodeSpaceError},
    modals::package::Package,
    path_utils::get_current_path,
};

use std::{
    fs,
    path::{Path, PathBuf},
};

use serde_json::{Map, Value};

use crate::errors::invalid_project::InvalidNodeProjectError;

pub fn is_package_exist(linked_packages: &Vec<Package>, path: &str) -> bool {
    let result = linked_packages.iter().find(|x| &x.path == path);

    match result {
        Some(_) => true,
        None => false,
    }
}

pub fn find_package_by_name(
    linked_packages: &Vec<Package>,
    effective_name: &str,
) -> Result<Package, NodeSpaceError> {
    let package = linked_packages.iter().find_map(|x| {
        let current_effective_name = match &x.alias {
            Some(value) => value,
            None => &x.name,
        };

        if current_effective_name == effective_name {
            return Some(x.clone());
        }

        None
    });

    if package.is_none() {
        return Err(NodeSpaceError::ConfigFileError(
            ConfigFileError::MissingLinkedPackage,
        ));
    }

    let package = package.unwrap();

    dbg!(&package);

    Ok(package)
}

pub fn get_package_json_path(path: &str) -> Result<PathBuf, InvalidNodeProjectError> {
    let package_json_path = Path::new(path).join("package.json");
    let is_valid_file = package_json_path.exists() && package_json_path.is_file();

    if !is_valid_file {
        return Err(InvalidNodeProjectError::MissingPackageJson);
    }

    Ok(package_json_path)
}

pub fn get_package_json_data(path: &str) -> Result<Map<String, Value>, InvalidNodeProjectError> {
    let package_json_path = get_package_json_path(path)?;

    let file_content = match fs::read_to_string(package_json_path) {
        Ok(value) => value,
        Err(_) => return Err(InvalidNodeProjectError::MissingPackageJson),
    };

    let json_value: Value = match serde_json::from_str(&file_content) {
        Ok(value) => value,
        Err(_) => return Err(InvalidNodeProjectError::MissingPackageJson),
    };

    match json_value {
        Value::Object(map) => Ok(map),
        _ => Err(InvalidNodeProjectError::InvalidPackageJson),
    }
}

// get base data of a node project from its package json file
// ```
// let (package_json_data, package_name, current_path) = get_base_package_data(None)?; // use
// current working dir
//
//
// let (package_json_data, package_name, current_path) = get_base_package_data(&path)?; // use
// custom path
// ```
pub fn get_base_package_data(
    package_path: Option<&str>,
) -> Result<(Map<String, Value>, String, String), InvalidNodeProjectError> {
    let current_working_dir = get_current_path()?;

    let current_path = match package_path {
        Some(path) => path.to_string(),
        None => current_working_dir,
    };

    let package_json_data = get_package_json_data(&current_path)?;
    let package_name_option = package_json_data.get("name").and_then(Value::as_str);

    if package_name_option.is_none() {
        return Err(InvalidNodeProjectError::MissingPackageJson.into());
    }

    let package_name = package_name_option.unwrap().to_string();

    Ok((package_json_data, package_name, current_path))
}

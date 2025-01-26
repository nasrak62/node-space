use std::{fs, path::Path};

use serde_json::Value;

use crate::{
    cli_manager::LinkArgs,
    errors::{invalid_project::InvalidNodeProjectError, node_space::NodeSpaceError},
    modals::config_file::ConfigFile,
    path_utils::get_current_path,
};

fn get_package_json_data(path: &str) -> Result<Value, InvalidNodeProjectError> {
    let package_json_path = Path::new(path).join("package.json");
    let is_valid_file = package_json_path.exists() && package_json_path.is_file();

    if !is_valid_file {
        return Err(InvalidNodeProjectError::MissingPackageJson);
    }

    let file_content = match fs::read_to_string(package_json_path) {
        Ok(value) => value,
        Err(_) => return Err(InvalidNodeProjectError::MissingPackageJson),
    };

    let json_value: Value = match serde_json::from_str(&file_content) {
        Ok(value) => value,
        Err(_) => return Err(InvalidNodeProjectError::MissingPackageJson),
    };

    Ok(json_value)
}

pub fn link_package(
    package_path: Option<&str>,
    package_name_alias: Option<String>,
) -> Result<bool, NodeSpaceError> {
    let mut config_file = ConfigFile::new()?;
    let current_working_dir = get_current_path()?;

    let current_path = match package_path {
        Some(path) => path.to_string(),
        None => current_working_dir,
    };

    let package_json_data = get_package_json_data(&current_path)?;
    let package_name = package_json_data.get("name").and_then(Value::as_str);

    if package_name.is_none() {
        return Err(InvalidNodeProjectError::MissingPackageJson.into());
    }

    config_file.add_linked_package(current_path, package_name.unwrap(), package_name_alias)?;

    Ok(true)
}

fn handle_show_linked_packages() -> Result<bool, NodeSpaceError> {
    let config_file = ConfigFile::new()?;

    for value in config_file.linked_pachages.iter() {
        println!("{}, at ({})", value.name, value.path);
    }

    Ok(true)
}

pub fn handle_link_command(link_args: &LinkArgs) -> Result<bool, NodeSpaceError> {
    if link_args.show {
        return Ok(handle_show_linked_packages()?);
    }

    let alias = &link_args.name;

    Ok(link_package(None, alias.clone())?)
}

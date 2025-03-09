use crate::{
    args::dependencies_args::UpdateDependencyArgs,
    errors::node_space::NodeSpaceError,
    modals::config_file::ConfigFile,
    package_utils::{get_base_package_data, get_package_json_path},
};

use serde_json::{json, Map, Value};
use std::fs;

use super::add::parse_package_name;

pub fn update_package_in_package_json(
    package_json_data: &mut Map<String, Value>,
    dependencies_name: &str,
    package_name: &str,
    package_version: &str,
) -> bool {
    let dependencies = package_json_data
        .get_mut(dependencies_name)
        .and_then(|deps| deps.as_object_mut());

    if dependencies.is_none() {
        return false;
    }

    let dependencies = dependencies.unwrap();

    if dependencies.get(package_name).is_none() {
        return false;
    }

    dependencies.insert(package_name.to_string(), json!(package_version));

    true
}

pub fn update_dependency_for_path(
    package_name: &str,
    package_version: &str,
    path: Option<&str>,
) -> Result<bool, NodeSpaceError> {
    let mut is_success = false;
    let (mut package_json_data, _, current_path) = get_base_package_data(path)?;
    let package_json_path = get_package_json_path(&current_path)?;

    for dependencies_name in ["dependencies", "devDependencies", "peerDependencies"] {
        let was_added = update_package_in_package_json(
            &mut package_json_data,
            dependencies_name,
            &package_name,
            package_version,
        );

        is_success = is_success || was_added;
    }

    if !is_success {
        return Ok(false);
    }

    let new_json_content = match serde_json::to_string_pretty(&package_json_data) {
        Ok(value) => value,
        Err(error) => {
            return Err(NodeSpaceError::InvalidPackageJsonAfterChanges(
                error.to_string(),
            ))
        }
    };

    match fs::write(package_json_path, new_json_content) {
        Ok(_) => (),
        Err(error) => {
            return Err(NodeSpaceError::InvalidPackageJsonAfterChanges(
                error.to_string(),
            ))
        }
    };

    println!(
        "Added dependency {}@{} to package.json",
        package_name, package_version
    );

    return Ok(true);
}

pub fn update_dependency(args: &UpdateDependencyArgs) -> Result<bool, NodeSpaceError> {
    let config_file = ConfigFile::new()?;

    let (package_name, package_version) = parse_package_name(&args.name)?;
    let package_name_str = &package_name.clone().to_owned();

    if args.group.is_none() {
        return update_dependency_for_path(package_name_str, package_version, None);
    }

    let group_name = &args.group.clone().unwrap();

    let current_groups = config_file.groups.get(group_name);

    if current_groups.is_none() {
        return Err(NodeSpaceError::GroupNameIsNotValid);
    }

    for package in current_groups.unwrap().iter() {
        update_dependency_for_path(package_name_str, package_version, Some(&package.path))?;
    }

    Ok(true)
}

use std::fs;

use crate::{
    args::dependencies_args::AddDependencyArgs,
    errors::node_space::NodeSpaceError,
    modals::config_file::ConfigFile,
    package_utils::{get_base_package_data, get_package_json_path},
};

use serde_json::{json, Map, Value};

pub fn add_package_to_package_json(
    package_json_data: &mut Map<String, Value>,
    dependencies_name: &str,
    package_name: &str,
    package_version: &str,
) -> bool {
    if package_json_data.get(dependencies_name).is_none() {
        package_json_data.insert(dependencies_name.to_string(), json!({}));
    }

    dbg!(&package_json_data);

    let dependencies = package_json_data
        .get_mut(dependencies_name)
        .and_then(|deps| deps.as_object_mut());

    dbg!(&dependencies);

    let dependencies = dependencies.unwrap();

    dependencies.insert(package_name.to_string(), json!(package_version));

    true
}

pub fn add_dependency_for_path(
    package_name: &str,
    package_version: &str,
    path: Option<&str>,
) -> Result<bool, NodeSpaceError> {
    let mut is_success = false;
    let (mut package_json_data, _, current_path) = get_base_package_data(path)?;
    let package_json_path = get_package_json_path(&current_path)?;

    for dependencies_name in ["dependencies"] {
        let was_added = add_package_to_package_json(
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

pub fn parse_package_name(name: &str) -> Result<(&str, &str), NodeSpaceError> {
    let package_info_list: Vec<&str> = name.split("@").collect();

    if package_info_list.len() < 2 {
        return Err(NodeSpaceError::InvalidPackageVersion);
    }

    let package_name = package_info_list[0];
    let package_version = package_info_list[1];

    Ok((package_name, package_version))
}

pub fn add_dependency(args: &AddDependencyArgs) -> Result<bool, NodeSpaceError> {
    let config_file = ConfigFile::new()?;

    let (package_name, package_version) = parse_package_name(&args.name)?;

    dbg!("{}, {}", package_name, package_version);

    if args.group.is_none() {
        return add_dependency_for_path(package_name, package_version, None);
    }

    let group_name = &args.group.clone().unwrap();

    let current_groups = config_file.groups.get(group_name);

    if current_groups.is_none() {
        return Err(NodeSpaceError::GroupNameIsNotValid);
    }

    for package in current_groups.unwrap().iter() {
        add_dependency_for_path(package_name, package_version, Some(&package.path))?;
    }

    Ok(true)
}

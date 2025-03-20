use std::{collections::HashMap, path::PathBuf};

use crate::{
    args::server_args::ConfigServerArgs,
    errors::node_space::NodeSpaceError,
    modals::{config_file::ConfigFile, package::Package, server_config::ServerConfig},
};

pub const DEFAULT_PORT: &str = "3000";

/// routes should have the format "route_name => project_name, route_name2 => project_name2"
/// we than transfrom it to
/// ```
/// {
/// "route_name": project_output_dir  -> default "dist"
/// "route_name2": project2_output_dir  -> default "dist"
/// }
///
/// ```
///
fn build_routes(
    routes: String,
    project_map: HashMap<String, Package>,
) -> Result<HashMap<String, String>, NodeSpaceError> {
    let routes_parts = routes.split(",").filter(|value| !value.is_empty());
    let routes_parts = routes_parts.collect::<Vec<&str>>();
    let mut map = HashMap::new();

    if routes_parts.is_empty() {
        return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
            "No routes found",
        )));
    }

    for route_part in routes_parts {
        if !route_part.contains("=>") {
            return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
                "Bad route format, missing '=>'",
            )));
        }

        let route_defenition_parts = route_part.split("=>").collect::<Vec<&str>>();

        if route_defenition_parts.len() != 2 {
            return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
                "Bad route format, should be  'route_name => project_name'",
            )));
        }

        let route_name = route_defenition_parts[0];
        let project_name = route_defenition_parts[1];

        if !project_map.contains_key(project_name.trim()) {
            return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
                "Unrecognized project name",
            )));
        }

        let project = project_map.get(project_name.trim());

        if project.is_none() {
            return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
                "Unrecognized project name",
            )));
        }

        let project = project.unwrap();
        let mut output_folder = project.path.clone();

        if !output_folder.ends_with("/") {
            output_folder += "/"
        }

        output_folder += &project.output_name;

        output_folder = output_folder.replace("//", "/");

        if !PathBuf::from(output_folder.clone()).exists() {
            return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
                "Output dir does not exist",
            )));
        }

        map.insert(route_name.trim().to_string(), output_folder);
    }

    Ok(map)
}

pub async fn handle_server_config(args: &ConfigServerArgs) -> Result<bool, NodeSpaceError> {
    let mut config_file = ConfigFile::new()?;
    let project_map = config_file.build_name_project_mapper();

    let port = args.port.clone().map_or(DEFAULT_PORT.to_string(), |v| v);

    let routes = build_routes(args.routes.clone(), project_map)?;
    let server_config = ServerConfig::new(args.name.clone(), port, routes, args.main_route.clone());

    config_file
        .server_config
        .entry(args.name.clone())
        .and_modify(|e| *e = server_config.clone())
        .or_insert(server_config);

    Ok(true)
}

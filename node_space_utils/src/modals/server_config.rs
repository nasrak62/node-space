use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{errors::node_space::NodeSpaceError, server::config::DEFAULT_PORT};

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct ServerConfig {
    pub port: String,
    pub name: String,
    pub routes: HashMap<String, String>,
    pub main_route: Option<String>,
}

impl ServerConfig {
    pub fn new(
        port: String,
        name: String,
        routes: HashMap<String, String>,
        main_route: Option<String>,
    ) -> Self {
        Self {
            port,
            name,
            routes,
            main_route,
        }
    }

    pub fn default(current_path: String) -> Self {
        let mut map = HashMap::new();

        let mut path = current_path;

        if !path.ends_with("/") {
            path += "/"
        }

        map.insert("/".to_string(), path + "dist");

        Self {
            port: DEFAULT_PORT.to_string(),
            name: "default".to_string(),
            routes: map,
            main_route: Some("/".to_string()),
        }
    }

    pub fn get_main_route_output_dir(&self) -> Result<String, NodeSpaceError> {
        if self.main_route.is_none() {
            let first_key = match self.routes.keys().next() {
                Some(value) => value,
                None => {
                    return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
                        "empty routes keys",
                    )))
                }
            };

            let first_value = match self.routes.get(first_key) {
                Some(value) => value,
                None => {
                    return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
                        "empty routes values",
                    )))
                }
            };

            return Ok(first_value.to_string());
        }
        match self.routes.get(&self.main_route.clone().unwrap()) {
            Some(value) => Ok(value.to_string()),
            None => {
                return Err(NodeSpaceError::InvalidRoutesConfig(String::from(
                    "the specified main_route does not have a matching route in the routes map",
                )))
            }
        }
    }
}

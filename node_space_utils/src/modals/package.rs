use serde::{Deserialize, Serialize};

fn default_dist() -> String {
    "dist".to_string()
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct Package {
    pub name: String,
    pub alias: Option<String>,
    pub path: String,

    #[serde(default = "default_dist")]
    pub output_name: String,
}

impl Package {
    pub fn new(
        path: String,
        name: String,
        alias: Option<String>,
        output_name: Option<String>,
    ) -> Self {
        let effective_output_name = match output_name {
            None => default_dist(),
            Some(value) => value,
        };

        Package {
            path,
            name,
            alias,
            output_name: effective_output_name,
        }
    }
}

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Package {
    pub name: String,
    pub alias: Option<String>,
    pub path: String,
}

impl Package {
    pub fn new(path: String, name: String, alias: Option<String>) -> Self {
        Package { path, name, alias }
    }
}

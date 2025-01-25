use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Package {
    pub name: String,
    pub path: String,
}

impl Package {
    pub fn new(path: String, name: String) -> Self {
        Package { path, name }
    }
}

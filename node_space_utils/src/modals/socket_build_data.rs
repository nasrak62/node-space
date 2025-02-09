use serde::{Deserialize, Serialize};

use super::package::Package;

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
pub struct SocketBuildData {
    pub project: Package,
    pub symlinks: Vec<Package>,
    pub watch_only_links: bool,
}

impl SocketBuildData {
    pub fn new(symlinks: Vec<Package>, project: Package, watch_only_links: bool) -> Self {
        SocketBuildData {
            symlinks,
            project,
            watch_only_links,
        }
    }
}

use std::env::{current_dir, var};
use std::fs;
use std::path::{Path, PathBuf};

use crate::errors::invalid_project::InvalidNodeProjectError;
use crate::errors::symlink::SymlinkError;

pub fn get_current_path() -> Result<String, InvalidNodeProjectError> {
    let path = match current_dir() {
        Ok(current_path) => current_path,
        Err(error) => return Err(InvalidNodeProjectError::Other(error.to_string())),
    };

    let cleaned_path = path.to_string_lossy().to_string();

    Ok(cleaned_path)
}

pub fn expand_tilde(path: &str) -> Result<PathBuf, InvalidNodeProjectError> {
    let home_dir = match var("HOME").ok().map(PathBuf::from) {
        Some(home_path) => home_path,
        None => {
            return Err(InvalidNodeProjectError::Other(
                "Can't get HOME env var".to_string(),
            ))
        }
    };

    let full_path = PathBuf::from(path.replace("~", &home_dir.to_string_lossy()));

    Ok(full_path)
}

pub fn get_package_path_from_node_modules(
    path: &str,
    package_name: &str,
) -> Result<String, SymlinkError> {
    let node_modules_package_path = Path::new(path).join("node_modules").join(package_name);

    if node_modules_package_path.exists() {
        dbg!("removed path");

        match fs::remove_dir_all(&node_modules_package_path) {
            Ok(_) => {}
            Err(error) => {
                return Err(SymlinkError::CantRemoveExistingDir(error.to_string()));
            }
        }
    }

    let value = node_modules_package_path
        .to_str()
        .map(String::from)
        .ok_or_else(|| SymlinkError::InvalidSymlink);

    value
}

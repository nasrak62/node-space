use std::path::PathBuf;

use crate::errors::node_space::NodeSpaceError;
use crate::errors::symlink::SymlinkError;
use crate::modals::config_file::ConfigFile;
use crate::modals::package::Package;
use crate::package_utils::find_package_by_name;

pub fn sync_links() -> Result<bool, NodeSpaceError> {
    let mut config_file = ConfigFile::new()?;

    let linked_packages = std::mem::take(&mut config_file.linked_packages);
    let original_length = linked_packages.len();
    let mut new_linked_packages: Vec<Package> = vec![];

    for package in linked_packages {
        let exist = match std::fs::exists(&package.path) {
            Ok(value) => value,
            Err(error) => {
                dbg!(error);

                return Err(NodeSpaceError::SymlinkError(SymlinkError::Other(
                    "can't check if path exists".to_string(),
                )));
            }
        };

        if exist {
            new_linked_packages.push(package);
        }
    }

    if original_length != new_linked_packages.len() {
        config_file.linked_packages = new_linked_packages;
        config_file.save()?;
    }

    let link_list = config_file.symlinks.clone();
    config_file.symlinks.clear();

    for (package_name, inner_links) in link_list {
        let mut new_symlinks: Vec<Package> = vec![];
        let package = find_package_by_name(&config_file.projects, &package_name)?;
        let path = &package.path;

        for inner_link in inner_links {
            let full_path = PathBuf::from(path)
                .join("node_modules")
                .join(inner_link.name);

            let metadata = match std::fs::symlink_metadata(full_path) {
                Ok(value) => value,
                Err(error) => {
                    dbg!(error);

                    return Err(NodeSpaceError::SymlinkError(SymlinkError::Other(
                        "can't check file metadata".to_string(),
                    )));
                }
            };

            if metadata.file_type().is_symlink() {
                new_symlinks.push(package.clone())
            }
        }

        config_file.symlinks.insert(package_name, new_symlinks);
    }

    config_file.save()?;

    Ok(true)
}

#[cfg(test)]
mod test;

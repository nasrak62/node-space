use std::os::unix::fs::symlink;

use crate::modals::package::Package;
use std::path::PathBuf;

use super::*;
use tempfile::NamedTempFile;

#[cfg(test)]
fn get_path() -> PathBuf {
    let file = NamedTempFile::new().unwrap();
    file.path().to_path_buf()
}

#[test]
fn test_remove_nonexistent_package_from_linked_packages() {
    let tmp_dir = tempfile::tempdir().unwrap();
    let pkg_path = tmp_dir.path().join("nonexistent_path");

    let package = Package::new(
        pkg_path.to_str().unwrap().to_string(),
        String::from("package-a"),
        None,
        None,
    );

    let file_path = Some(get_path());

    let mut config = ConfigFile::new(file_path.clone()).unwrap();
    config.linked_packages.push(package);

    config.save().unwrap();

    dbg!(&config);

    tmp_dir.close().unwrap();

    let result = config.sync_linked_packages();
    assert!(result.is_ok());

    let updated_config = ConfigFile::new(file_path).unwrap();

    dbg!(&updated_config);
    assert!(updated_config.linked_packages.is_empty());
}

#[test]
fn test_remove_unlinked_package_from_symlinks() {
    let tmp_project = tempfile::tempdir().unwrap();
    let tmp_package = tempfile::tempdir().unwrap();
    let package_name = "package-b";

    let file_path = Some(get_path());

    let package = Package::new(
        tmp_package.path().to_str().unwrap().to_string(),
        String::from(package_name),
        None,
        None,
    );

    let project = Package::new(
        tmp_project.path().to_str().unwrap().to_string(),
        String::from("project-x"),
        None,
        None,
    );

    // Setup config
    let mut config = ConfigFile::new(file_path.clone()).unwrap();
    config.linked_packages.push(package.clone());
    config.projects.push(project.clone());
    config
        .symlinks
        .insert(project.name.clone(), vec![package.clone()]);

    // Create a node_modules dir without symlink
    let node_modules_path = tmp_project.path().join("node_modules");
    std::fs::create_dir_all(&node_modules_path).unwrap();

    config.save().unwrap();

    let result = config.sync_links();
    assert!(result.is_ok());
    let updated_config = ConfigFile::new(file_path).unwrap();

    assert!(updated_config.symlinks[&project.name].is_empty());
}

#[test]
fn test_keeps_linked_symlink() {
    let tmp_project = tempfile::tempdir().unwrap();
    let tmp_package = tempfile::tempdir().unwrap();
    let file_path = Some(get_path());
    let package_name = "package-c";

    let package = Package::new(
        tmp_package.path().to_str().unwrap().to_string(),
        String::from(package_name),
        None,
        None,
    );

    let project = Package::new(
        tmp_project.path().to_str().unwrap().to_string(),
        String::from("project-y"),
        None,
        None,
    );

    let mut config = ConfigFile::new(file_path.clone()).unwrap();
    config.linked_packages.push(package.clone());
    config.projects.push(project.clone());
    config
        .symlinks
        .insert(project.name.clone(), vec![package.clone()]);

    let node_modules_path = tmp_project.path().join("node_modules");
    std::fs::create_dir_all(&node_modules_path).unwrap();

    let symlink_path = node_modules_path.join(package_name);
    symlink(&tmp_package.path(), &symlink_path).unwrap();

    config.save().unwrap();

    let result = config.sync_links();
    assert!(result.is_ok());
    let updated_config = ConfigFile::new(file_path).unwrap();
    assert_eq!(updated_config.symlinks[&project.name].len(), 1);
}

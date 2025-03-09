use std::sync::{Arc, Mutex};

use crate::modals::{coordinator::Coordinator, socket_build_data::SocketBuildData};

pub fn process_stream_request(shared_lock_coordinator: &Arc<Mutex<Coordinator>>, data_str: &str) {
    let data: SocketBuildData = match serde_json::from_str(&data_str) {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{}", error.to_string());

            return;
        }
    };

    let mut coordinator = match shared_lock_coordinator.lock() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{}", error.to_string());

            return;
        }
    };

    for package in data.symlinks.iter() {
        if coordinator.watchers_target.contains(&package.path) {
            continue;
        }

        coordinator.watchers_target.push(package.path.clone());

        let current_entry = coordinator
            .dependencies_to_projects_map
            .entry(package.path.clone())
            .or_insert_with(Vec::new);

        if !current_entry.contains(&data.project.path) {
            current_entry.push(data.project.path.clone());
        }

        let current_entry = coordinator
            .projects_to_dependencies_map
            .entry(data.project.path.clone())
            .or_insert_with(Vec::new);

        if !current_entry.contains(&package.path) {
            current_entry.push(package.path.clone());
        }
    }

    if !data.watch_only_links && !coordinator.watchers_target.contains(&data.project.path) {
        coordinator.watchers_target.push(data.project.path)
    }
}

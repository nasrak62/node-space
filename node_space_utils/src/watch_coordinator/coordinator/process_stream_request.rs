use std::sync::{Arc, Mutex};

use crate::modals::{coordinator::Coordinator, socket_build_data::SocketBuildData};

use super::log_utils::{log_to_file, LogFile};

pub fn process_stream_request(
    shared_lock_coordinator: &Arc<Mutex<Coordinator>>,
    data_str: &str,
    shared_logger: &LogFile,
) {
    let data: SocketBuildData = match serde_json::from_str(&data_str) {
        Ok(value) => value,
        Err(error) => {
            let _ = log_to_file(
                &format!(
                    "error build socket data from message: {},\ndata: {}",
                    error.to_string(),
                    &data_str
                ),
                shared_logger,
            );

            return;
        }
    };

    let mut coordinator = match shared_lock_coordinator.lock() {
        Ok(value) => value,
        Err(error) => {
            let _ = log_to_file(
                &format!("error getting coordinator: {}", error.to_string()),
                shared_logger,
            );

            return;
        }
    };

    for package in data.symlinks.iter() {
        if coordinator
            .watchers_target
            .iter()
            .any(|p| p.path == package.path)
        {
            continue;
        }

        let _ = log_to_file(
            &format!("adding new package to watchers_target: {}", &package.name),
            shared_logger,
        );

        coordinator.watchers_target.push(package.clone());

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

    if !data.watch_only_links
        && !coordinator
            .watchers_target
            .iter()
            .any(|p| p.path == data.project.path)
    {
        let _ = log_to_file(
            &format!(
                "adding new package to watchers_target: {}",
                &data.project.name
            ),
            shared_logger,
        );

        coordinator.watchers_target.push(data.project)
    }
}

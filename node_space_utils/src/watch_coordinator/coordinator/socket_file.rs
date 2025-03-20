use std::{fs::OpenOptions, path::Path};

use crate::{
    errors::node_space::NodeSpaceError,
    watch_coordinator::{
        client::constants::COORDINATOR_SOCKET_PATH, coordinator::log_utils::log_to_file,
    },
};

use super::log_utils::LogFile;

pub fn delete_socket_file(shared_logger: &LogFile) {
    if !Path::new(&COORDINATOR_SOCKET_PATH).exists() {
        return;
    }

    let removed_result = std::fs::remove_file(&COORDINATOR_SOCKET_PATH);

    if removed_result.is_err() {
        let _ = log_to_file("Error removing socket file", shared_logger);
    }
}

pub fn create_socket_file(shared_logger: &LogFile) -> Result<(), NodeSpaceError> {
    if Path::new(&COORDINATOR_SOCKET_PATH).exists() {
        delete_socket_file(shared_logger);

        return Ok(());
    }

    match OpenOptions::new()
        .append(true)
        .create(true)
        .open(&COORDINATOR_SOCKET_PATH)
    {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{}", error);

            return Err(NodeSpaceError::CantCreateSocketFile(error.to_string()));
        }
    };

    log_to_file("created socket file", shared_logger)?;

    Ok(())
}

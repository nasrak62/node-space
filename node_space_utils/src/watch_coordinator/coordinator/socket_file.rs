use std::{fs::OpenOptions, path::Path};

use crate::{
    errors::node_space::NodeSpaceError,
    watch_coordinator::client::constants::COORDINATOR_SOCKET_PATH,
};

pub fn delete_socket_file() {
    if !Path::new(&COORDINATOR_SOCKET_PATH).exists() {
        return;
    }

    let removed_result = std::fs::remove_file(&COORDINATOR_SOCKET_PATH);

    if removed_result.is_err() {
        eprintln!("Error removing socket file");
    }
}

pub fn create_socket_file() -> Result<(), NodeSpaceError> {
    if Path::new(&COORDINATOR_SOCKET_PATH).exists() {
        delete_socket_file();

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

    Ok(())
}

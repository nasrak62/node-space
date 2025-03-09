use std::{
    fs::OpenOptions,
    io::Write,
    path::Path,
    sync::{Arc, Mutex},
};

use crate::errors::node_space::NodeSpaceError;

pub const COORDINATOR_LOG_FILE: &str = "/tmp/node-space-coordinator.log";

pub type LogFile = Arc<Mutex<std::fs::File>>;

pub fn delete_logging_file() {
    if !Path::new(&COORDINATOR_LOG_FILE).exists() {
        return;
    }

    let removed_result = std::fs::remove_file(&COORDINATOR_LOG_FILE);

    if removed_result.is_err() {
        eprintln!("Error removing logging file");
    }
}

pub fn create_logging_file() -> Result<LogFile, NodeSpaceError> {
    delete_logging_file();

    let file = match OpenOptions::new()
        .append(true)
        .create(true)
        .open(COORDINATOR_LOG_FILE)
    {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{}", error);

            return Err(NodeSpaceError::CantCreateLogFile(error.to_string()));
        }
    };

    let shared_file = Arc::new(Mutex::new(file));

    Ok(shared_file)
}

pub fn log_to_file(message: &str, shared_file: &LogFile) -> Result<(), NodeSpaceError> {
    dbg!(message);

    let mut file = match shared_file.lock() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("{}", error);

            return Err(NodeSpaceError::CantWriteLogFile(error.to_string()));
        }
    };

    let effective_message = message.to_string() + "\n";

    match file.write_all(effective_message.as_bytes()) {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);

            return Err(NodeSpaceError::CantWriteLogFile(error.to_string()));
        }
    };

    match file.flush() {
        Ok(_) => {}
        Err(error) => {
            eprintln!("{}", error);

            return Err(NodeSpaceError::CantWriteLogFile(error.to_string()));
        }
    };

    Ok(())
}

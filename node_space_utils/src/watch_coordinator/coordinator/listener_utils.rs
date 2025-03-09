use std::os::unix::net::UnixListener;

use crate::{
    errors::{node_space::NodeSpaceError, socket::SocketError},
    watch_coordinator::client::constants::COORDINATOR_SOCKET_PATH,
};

use super::log_utils::{log_to_file, LogFile};

pub fn init_listener(shared_logger: &LogFile) -> Result<UnixListener, NodeSpaceError> {
    let listener = match UnixListener::bind(COORDINATOR_SOCKET_PATH) {
        Ok(value) => value,
        Err(error) => {
            return Err(NodeSpaceError::SocketError(
                SocketError::ErrorConnectingToSocket(error.to_string()),
            ))
        }
    };

    match listener.set_nonblocking(true) {
        Ok(_) => {}
        Err(error) => {
            return Err(NodeSpaceError::SocketError(
                SocketError::ErrorConnectingToSocket(error.to_string()),
            ))
        }
    }

    log_to_file("created listener", shared_logger)?;

    Ok(listener)
}

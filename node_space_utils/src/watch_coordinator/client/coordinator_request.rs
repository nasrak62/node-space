use serde_json::to_string;
use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::{
    errors::{node_space::NodeSpaceError, socket::SocketError},
    modals::socket_build_data::SocketBuildData,
};

use super::constants::COORDINATOR_SOCKET_PATH;

pub fn send_coordinator_request(socket_data: SocketBuildData) -> Result<bool, NodeSpaceError> {
    let mut stream = match UnixStream::connect(COORDINATOR_SOCKET_PATH) {
        Ok(value) => value,
        Err(error) => {
            return Err(NodeSpaceError::SocketError(
                SocketError::ErrorConnectingToSocket(error.to_string()),
            ))
        }
    };

    let socket_data_string = match to_string(&socket_data) {
        Ok(value) => value,
        Err(error) => {
            return Err(NodeSpaceError::SocketError(
                SocketError::ErrorSendingDataInSocket(error.to_string()),
            ))
        }
    };

    match writeln!(stream, "{}", socket_data_string) {
        Ok(_) => Ok(true),
        Err(error) => {
            return Err(NodeSpaceError::SocketError(
                SocketError::ErrorSendingDataInSocket(error.to_string()),
            ))
        }
    }
}

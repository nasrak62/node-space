use serde_json;

use std::io::Write;
use std::os::unix::net::UnixStream;

use crate::{
    errors::{node_space::NodeSpaceError, socket::SocketError},
    modals::socket_build_data::SocketBuildData,
    retry::retry,
};

use super::client::constants::COORDINATOR_SOCKET_PATH;

// TODO: check if build success?
pub fn send_data_to_coordinator(data: SocketBuildData) -> Result<(), NodeSpaceError> {
    let stream = retry(
        || match UnixStream::connect(COORDINATOR_SOCKET_PATH) {
            Ok(value) => Ok(value),
            Err(error) => Err(SocketError::ErrorConnectingToSocket(error.to_string()).into()),
        },
        None,
        None,
    );

    let mut stream = stream.unwrap();

    let project_str = match serde_json::to_string(&data) {
        Ok(value) => value,
        Err(error) => return Err(SocketError::ErrorSendingDataInSocket(error.to_string()).into()),
    };

    match writeln!(stream, "{}", project_str) {
        Ok(_) => (),
        Err(error) => return Err(SocketError::ErrorSendingDataInSocket(error.to_string()).into()),
    };

    Ok(())
}

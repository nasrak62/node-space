use std::{os::unix::net::UnixStream, path::Path, time::Duration};

use crate::{
    errors::socket::SocketError, watch_coordinator::coordinator::socket_file::delete_socket_file,
};

const STREAM_TIMEOUT: Option<Duration> = Some(Duration::from_millis(100));

pub fn is_socket_active(socket_path: &str) -> Result<bool, SocketError> {
    dbg!("checking if socket active");

    if !Path::new(socket_path).exists() {
        dbg!("socket path does not exist there is not coordiantor");

        return Ok(false);
    }

    let stream = match UnixStream::connect(socket_path) {
        Ok(stream) => stream,
        Err(_) => {
            dbg!("can't connect to socket, not active");

            delete_socket_file();

            return Ok(false);
        }
    };

    match stream.set_read_timeout(STREAM_TIMEOUT) {
        Ok(_) => (),
        Err(error) => {
            dbg!("socket read timeout");

            return Err(SocketError::ErrorConnectingToSocket(error.to_string()));
        }
    };

    match stream.set_write_timeout(STREAM_TIMEOUT) {
        Ok(_) => (),
        Err(error) => {
            dbg!("socket write timeout");

            return Err(SocketError::ErrorConnectingToSocket(error.to_string()));
        }
    };

    drop(stream);

    dbg!("dropped is active check stream");

    Ok(true)
}

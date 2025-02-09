use std::{os::unix::net::UnixStream, path::Path, time::Duration};

use crate::errors::socket::SocketError;

const STREAM_TIMEOUT: Option<Duration> = Some(Duration::from_millis(100));

pub fn is_socket_active(socket_path: &str) -> Result<bool, SocketError> {
    if !Path::new(socket_path).exists() {
        return Ok(false);
    }

    let stream = match UnixStream::connect(socket_path) {
        Ok(stream) => stream,
        Err(_) => return Ok(false),
    };

    match stream.set_read_timeout(STREAM_TIMEOUT) {
        Ok(_) => (),
        Err(error) => return Err(SocketError::ErrorConnectingToSocket(error.to_string())),
    };

    match stream.set_write_timeout(STREAM_TIMEOUT) {
        Ok(_) => (),
        Err(error) => return Err(SocketError::ErrorConnectingToSocket(error.to_string())),
    };

    drop(stream);

    Ok(true)
}

use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SocketError {
    ErrorConnectingToSocket(String),
    ErrorSendingDataInSocket(String),
    Other(String),
}

impl fmt::Display for SocketError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SocketError::ErrorConnectingToSocket(ref message) => {
                write!(f, "Socket error, can't connect to socket: {}", message)
            }
            SocketError::ErrorSendingDataInSocket(ref message) => {
                write!(f, "Socket error, can't send data to socket: {}", message)
            }

            SocketError::Other(ref message) => {
                write!(f, "Socket error: {}", message)
            }
        }
    }
}

impl Error for SocketError {}

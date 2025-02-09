use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum BuildError {
    CantSpwnBuilCommand(String),
    CantWaitForChildProcess(String),
    ChildCommandFailed(String),
    Other(String),
}

impl fmt::Display for BuildError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            BuildError::CantSpwnBuilCommand(ref message) => {
                write!(f, "Can't spwn build command: {}", message)
            }
            BuildError::CantWaitForChildProcess(ref message) => {
                write!(f, "Can't wait for child process: {}", message)
            }
            BuildError::ChildCommandFailed(ref message) => {
                write!(f, "Child command failed: {}", message)
            }
            BuildError::Other(ref message) => {
                write!(f, "Error building project: {}", message)
            }
        }
    }
}

impl Error for BuildError {}

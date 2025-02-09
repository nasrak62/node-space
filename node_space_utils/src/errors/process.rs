use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ProcessError {
    ErrorCheckingProcessIsRunning(String),
    Other(String),
}

impl fmt::Display for ProcessError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ProcessError::ErrorCheckingProcessIsRunning(ref message) => {
                write!(
                    f,
                    "Proccess error, can't check if old process is running: {}",
                    message
                )
            }

            ProcessError::Other(ref message) => {
                write!(f, "Process error: {}", message)
            }
        }
    }
}

impl Error for ProcessError {}

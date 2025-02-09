use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum WatcherError {
    CantCreateWatcher(String),
    Other(String),
}

impl fmt::Display for WatcherError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            WatcherError::CantCreateWatcher(ref message) => {
                write!(f, "Watch error, can't add watcher: {}", message)
            }
            WatcherError::Other(ref message) => {
                write!(f, "Watch error: {}", message)
            }
        }
    }
}

impl Error for WatcherError {}

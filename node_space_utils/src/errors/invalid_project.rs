use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum InvalidNodeProjectError {
    MissingPackageJson,
    InvalidDirectory,
    Other(String),
}

impl fmt::Display for InvalidNodeProjectError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidNodeProjectError::MissingPackageJson => {
                write!(f, "The node project is invalid: Missing package.json")
            }
            InvalidNodeProjectError::InvalidDirectory => {
                write!(f, "The node project is invalid: Invalid directory")
            }
            InvalidNodeProjectError::Other(ref message) => {
                write!(f, "Invalid node project: {}", message)
            }
        }
    }
}

impl Error for InvalidNodeProjectError {}

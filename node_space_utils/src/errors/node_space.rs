use std::error::Error;
use std::fmt;

use super::{
    config_file::ConfigFileError, invalid_project::InvalidNodeProjectError, symlink::SymlinkError,
};

#[derive(Debug)]
pub enum NodeSpaceError {
    ConfigFileError(ConfigFileError),
    InvalidNodeProjectError(InvalidNodeProjectError),
    SymlinkError(SymlinkError),
}

impl fmt::Display for NodeSpaceError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            NodeSpaceError::ConfigFileError(err) => {
                write!(f, "Config error: {}", err)
            }

            NodeSpaceError::InvalidNodeProjectError(err) => {
                write!(f, "Project error: {}", err)
            }
            NodeSpaceError::SymlinkError(err) => {
                write!(f, "Symlink error: {}", err)
            }
        }
    }
}

impl Error for NodeSpaceError {}

impl From<ConfigFileError> for NodeSpaceError {
    fn from(err: ConfigFileError) -> Self {
        NodeSpaceError::ConfigFileError(err)
    }
}

impl From<InvalidNodeProjectError> for NodeSpaceError {
    fn from(err: InvalidNodeProjectError) -> Self {
        NodeSpaceError::InvalidNodeProjectError(err)
    }
}

impl From<SymlinkError> for NodeSpaceError {
    fn from(err: SymlinkError) -> Self {
        NodeSpaceError::SymlinkError(err)
    }
}

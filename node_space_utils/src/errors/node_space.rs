use std::error::Error;
use std::fmt;

use super::{
    build::BuildError, config_file::ConfigFileError, invalid_project::InvalidNodeProjectError,
    process::ProcessError, socket::SocketError, symlink::SymlinkError, watcher::WatcherError,
};

#[derive(Debug)]
pub enum NodeSpaceError {
    ConfigFileError(ConfigFileError),
    InvalidNodeProjectError(InvalidNodeProjectError),
    InvalidPackageVersion,
    GroupNameIsNotValid,
    InvalidPackageJsonAfterChanges(String),
    SymlinkError(SymlinkError),
    BuildError(BuildError),
    ProcessError(ProcessError),
    SocketError(SocketError),
    WatcherError(WatcherError),
    CantOpenPIDFile(String),
    CantParsePIDNumber(String),
    CantStartCoordinator(String),
    CantWriteToPIDFile(String),
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

            NodeSpaceError::BuildError(err) => {
                write!(f, "build error: {}", err)
            }
            NodeSpaceError::ProcessError(err) => {
                write!(f, "ProcessError error: {}", err)
            }

            NodeSpaceError::SocketError(err) => {
                write!(f, "SocketError error: {}", err)
            }

            NodeSpaceError::WatcherError(err) => {
                write!(f, "WatcherError error: {}", err)
            }

            NodeSpaceError::InvalidPackageVersion => {
                write!(f, "The Specified package has bad format: 'name@version'")
            }
            NodeSpaceError::InvalidPackageJsonAfterChanges(ref message) => {
                write!(
                    f,
                    "The package json data is not valid after the recent changes, can't jsonify: {}",
                    message
                )
            }

            NodeSpaceError::GroupNameIsNotValid => {
                write!(f, "The group name you have entered in not in the config file, please create this group first")
            }
            NodeSpaceError::CantOpenPIDFile(ref message) => {
                write!(f, "Can't open PID file: {}", message)
            }
            NodeSpaceError::CantParsePIDNumber(ref message) => {
                write!(f, "Can't Parse pid number: {}", message)
            }

            NodeSpaceError::CantStartCoordinator(ref message) => {
                write!(f, "Can't start coordinator: {}", message)
            }

            NodeSpaceError::CantWriteToPIDFile(ref message) => {
                write!(f, "Can't write to pid file: {}", message)
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

impl From<BuildError> for NodeSpaceError {
    fn from(err: BuildError) -> Self {
        NodeSpaceError::BuildError(err)
    }
}

impl From<ProcessError> for NodeSpaceError {
    fn from(err: ProcessError) -> Self {
        NodeSpaceError::ProcessError(err)
    }
}

impl From<SocketError> for NodeSpaceError {
    fn from(err: SocketError) -> Self {
        NodeSpaceError::SocketError(err)
    }
}

impl From<WatcherError> for NodeSpaceError {
    fn from(err: WatcherError) -> Self {
        NodeSpaceError::WatcherError(err)
    }
}

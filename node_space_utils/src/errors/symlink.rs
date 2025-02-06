use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum SymlinkError {
    InvalidSymlink,
    MissingLinkToTargetName,
    CantRemoveExistingDir(String),
    Other(String),
}

impl fmt::Display for SymlinkError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SymlinkError::InvalidSymlink => {
                write!(f, "Invalid symlink")
            }
            SymlinkError::MissingLinkToTargetName => {
                write!(f, "Missing Link To Target Name")
            }

            SymlinkError::CantRemoveExistingDir(ref message) => {
                write!(f, "Symlink error, can't remove existing dir: {}", message)
            }
            SymlinkError::Other(ref message) => {
                write!(f, "Symlink error: {}", message)
            }
        }
    }
}

impl Error for SymlinkError {}

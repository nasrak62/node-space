use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum ConfigFileError {
    CantFindFile,
    CantCreateDir,
    CantDesirialize,
    CantSerialize(String),
    MissingLinkedPackage,
    FailedToCreateSymLink(String),
    Other(String),
}

impl fmt::Display for ConfigFileError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ConfigFileError::CantFindFile => {
                write!(f, "Can't find the config file")
            }
            ConfigFileError::CantDesirialize => {
                write!(f, "Can't Deserialize the config file")
            }
            ConfigFileError::CantSerialize(ref message) => {
                write!(f, "Can't Serialize the config file: {}", message)
            }

            ConfigFileError::CantCreateDir => {
                write!(f, "Can't create config dir")
            }
            ConfigFileError::MissingLinkedPackage => {
                write!(f, "Can't link to a package that was not registered")
            }
            ConfigFileError::FailedToCreateSymLink(ref message) => {
                write!(f, "Can't create symlink: {}", message)
            }
            ConfigFileError::Other(ref message) => {
                write!(f, "Invalid config file: {}", message)
            }
        }
    }
}

impl Error for ConfigFileError {}

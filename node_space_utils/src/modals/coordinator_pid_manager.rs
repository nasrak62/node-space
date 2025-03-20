use std::fs;
use std::path::Path;

use crate::errors::node_space::NodeSpaceError;

pub const COORDINATOR_PID_PATH: &str = "/tmp/node-space-coordinator.pid";

#[derive(Copy, Clone)]
pub struct CoordinatorPIDManager;

impl CoordinatorPIDManager {
    pub fn new() -> Self {
        Self
    }

    pub fn is_exists(&self) -> bool {
        Path::new(COORDINATOR_PID_PATH).exists()
    }

    pub fn read_pid(&self) -> Result<u32, NodeSpaceError> {
        match fs::read_to_string(COORDINATOR_PID_PATH) {
            Ok(value) => match value.trim().parse::<u32>() {
                Ok(number_value) => Ok(number_value),
                Err(error) => Err(NodeSpaceError::CantParsePIDNumber(error.to_string())),
            },
            Err(error) => Err(NodeSpaceError::CantOpenPIDFile(error.to_string())),
        }
    }

    pub fn write_pid(&self, pid: u32) -> Result<(), NodeSpaceError> {
        match fs::write(COORDINATOR_PID_PATH, pid.to_string()) {
            Ok(_) => Ok(()),
            Err(error) => Err(NodeSpaceError::CantWriteToPIDFile(error.to_string())),
        }
    }
}

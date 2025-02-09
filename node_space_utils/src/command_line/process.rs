use std::process::Command;

use crate::errors::process::ProcessError;

pub fn is_process_running(pid: u32) -> Result<bool, ProcessError> {
    let status = Command::new("kill").arg("-0").arg(pid.to_string()).status();

    match status {
        Ok(value) => Ok(value.success()),
        Err(error) => Err(ProcessError::ErrorCheckingProcessIsRunning(
            error.to_string(),
        )),
    }
}

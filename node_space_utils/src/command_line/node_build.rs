use std::{
    io::Error,
    process::{Child, Command, Stdio},
};

use crate::errors::build::BuildError;

pub fn handle_command_result(spwn: Result<Child, Error>) -> Result<bool, BuildError> {
    let mut child = match spwn {
        Ok(value) => value,
        Err(error) => return Err(BuildError::CantSpwnBuilCommand(error.to_string())),
    };

    let status = match child.wait() {
        Ok(value) => value,
        Err(error) => return Err(BuildError::CantWaitForChildProcess(error.to_string())),
    };

    if !status.success() {
        return Err(BuildError::ChildCommandFailed(status.to_string()));
    }

    Ok(true)
}

pub fn run_node_command(path: &str, command_name: &str) -> Result<bool, BuildError> {
    let spwn = Command::new("npm")
        .arg("run")
        .arg(command_name)
        .current_dir(path)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    handle_command_result(spwn)
}

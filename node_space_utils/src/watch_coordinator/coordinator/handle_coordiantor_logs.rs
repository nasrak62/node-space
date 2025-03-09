use std::{
    io::Error,
    process::{Child, Command, Stdio},
};

use crate::{
    args::coordinator_args::CoordinatorLogArgs, command_line::node_build::handle_command_result,
    errors::node_space::NodeSpaceError,
};

use super::log_utils::COORDINATOR_LOG_FILE;

fn create_log_command(log_args: &CoordinatorLogArgs) -> Result<Child, Error> {
    if log_args.watch {
        let spwn = Command::new("cat")
            .arg(COORDINATOR_LOG_FILE)
            .stdout(Stdio::inherit())
            .stderr(Stdio::inherit())
            .spawn();

        return spwn;
    }

    let spwn = Command::new("cat")
        .arg(COORDINATOR_LOG_FILE)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .spawn();

    spwn
}

pub fn handle_coordinator_logs(log_args: &CoordinatorLogArgs) -> Result<bool, NodeSpaceError> {
    let spwn = create_log_command(log_args);

    match handle_command_result(spwn) {
        Ok(value) => Ok(value),
        Err(error) => Err(error.into()),
    }
}

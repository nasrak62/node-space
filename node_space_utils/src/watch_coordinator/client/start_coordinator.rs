use libc::{close, exit, fork, setsid, STDERR_FILENO, STDIN_FILENO, STDOUT_FILENO};

use std::{
    os::unix::process::CommandExt,
    process::{Command, Stdio},
};

use crate::{
    command_line::process::is_process_running,
    errors::{node_space::NodeSpaceError, watcher::WatcherError},
    modals::{coordinator_pid_manager::CoordinatorPIDManager, socket_build_data::SocketBuildData},
    socket::socket_active_utils::is_socket_active,
    watch_coordinator::coordinator_communication::send_data_to_coordinator,
    watcher_utils::add_local_watcher,
};

use crate::watch_coordinator::client::constants::COORDINATOR_SOCKET_PATH;

pub fn is_coordinator_running() -> Result<bool, NodeSpaceError> {
    let pid_file_manager = CoordinatorPIDManager::new();

    if !pid_file_manager.is_exists() {
        return Ok(false);
    }

    let pid = pid_file_manager.read_pid()?;

    let process_active = is_process_running(pid)?;

    dbg!(process_active);

    if !process_active {
        return Ok(false);
    }

    let socket_active = is_socket_active(COORDINATOR_SOCKET_PATH)?;

    dbg!(socket_active);

    Ok(socket_active)
}

pub fn start_coordinator() -> Result<(), NodeSpaceError> {
    if is_coordinator_running()? {
        dbg!("coordinator is running skiping init...");

        return Ok(());
    }

    dbg!("spawning node-space");

    unsafe {
        let pid = fork();

        if pid < 0 {
            return Err(NodeSpaceError::CantStartCoordinator(
                "Error forking".to_string(),
            ));
        }

        if pid > 0 {
            dbg!("parent continue");

            return Ok(());
        }

        if setsid() < 0 {
            return Err(NodeSpaceError::CantStartCoordinator(
                "setsid error".to_string(),
            ));
        }

        let pid2 = fork();

        if pid2 < 0 {
            return Err(NodeSpaceError::CantStartCoordinator(
                "Error forking second time".to_string(),
            ));
        }

        if pid2 > 0 {
            exit(0)
        }

        close(STDIN_FILENO);
        close(STDOUT_FILENO);
        close(STDERR_FILENO);

        Command::new("node-space")
            .arg("coordinator")
            .arg("start")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .stdin(Stdio::null())
            .exec();

        return Err(NodeSpaceError::CantStartCoordinator(
            "Exec command failed".to_string(),
        ));
    }
}

pub fn add_local_project_watcher(data: SocketBuildData) -> Result<(), NodeSpaceError> {
    match add_local_watcher(data) {
        Ok(_) => Ok(()),
        Err(error) => Err(NodeSpaceError::WatcherError(WatcherError::Other(
            error.to_string(),
        ))),
    }
}

pub fn request_build_watcher_for_project(data: SocketBuildData) -> Result<bool, NodeSpaceError> {
    dbg!("request_build_watcher_for_project");

    send_data_to_coordinator(data)?;

    Ok(true)
}

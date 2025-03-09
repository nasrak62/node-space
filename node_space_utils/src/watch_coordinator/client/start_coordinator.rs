use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use crate::{
    command_line::process::is_process_running,
    errors::{node_space::NodeSpaceError, watcher::WatcherError},
    modals::socket_build_data::SocketBuildData,
    socket::socket_active_utils::is_socket_active,
    watch_coordinator::coordinator_communication::send_data_to_coordinator,
    watcher_utils::add_local_watcher,
};

use crate::watch_coordinator::client::constants::{COORDINATOR_PID_PATH, COORDINATOR_SOCKET_PATH};

pub fn is_coordinator_running() -> Result<bool, NodeSpaceError> {
    if !Path::new(COORDINATOR_PID_PATH).exists() {
        return Ok(false);
    }

    let pid = match fs::read_to_string(COORDINATOR_PID_PATH) {
        Ok(value) => match value.trim().parse::<u32>() {
            Ok(number_value) => number_value,
            Err(error) => return Err(NodeSpaceError::CantParsePIDNumber(error.to_string())),
        },
        Err(error) => return Err(NodeSpaceError::CantOpenPIDFile(error.to_string())),
    };

    let process_active = is_process_running(pid)?;

    dbg!("process_active:", process_active);

    let socket_active = is_socket_active(COORDINATOR_SOCKET_PATH)?;

    dbg!("socket_active:", socket_active);

    let is_running = process_active && socket_active;

    Ok(is_running)
}

pub fn start_coordinator() -> Result<(), NodeSpaceError> {
    if is_coordinator_running()? {
        dbg!("coordinator is running skiping init...");

        return Ok(());
    }

    dbg!("spawning node-space");

    let child_result = Command::new("setsid")
        .arg("node-space")
        .arg("coordinator")
        .arg("start")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn();

    let child = match child_result {
        Ok(value) => value,
        Err(error) => return Err(NodeSpaceError::CantStartCoordinator(error.to_string())),
    };

    let child_id = child.id().to_string();

    match fs::write(COORDINATOR_PID_PATH, &child_id) {
        Ok(_) => {
            dbg!("Coordinator started with PID: {}", &child_id);
            Ok(())
        }
        Err(error) => Err(NodeSpaceError::CantWriteToPIDFile(error.to_string())),
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

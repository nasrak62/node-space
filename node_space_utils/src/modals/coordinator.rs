use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use notify::RecommendedWatcher;
use notify_debouncer_full::{Debouncer, RecommendedCache};

use crate::watch_coordinator::coordinator::handle_signals::handle_termination_signals;
use crate::watch_coordinator::coordinator::listener_utils::init_listener;
use crate::watch_coordinator::coordinator::log_utils::{create_logging_file, log_to_file};
use crate::watch_coordinator::coordinator::socket_file::create_socket_file;
use crate::watch_coordinator::coordinator::thread_utils::send_thread_kill_signal;
use crate::{
    errors::{node_space::NodeSpaceError, socket::SocketError},
    watcher_utils::create_watcher_instance,
};

use super::coordinator_listener::CoordinatorListener;
use super::coordinator_pid_manager::CoordinatorPIDManager;
use super::coordinator_updates_manager::CoordinatorUpdatesManager;
use super::coordinator_watcher_handler::CoordinatorWatcherHandler;
use super::package::Package;

pub type SharedWatcherClone = Arc<Mutex<Debouncer<RecommendedWatcher, RecommendedCache>>>;

pub struct Coordinator {
    pub watchers_target: Vec<Package>,
    pub active_watchers: Vec<String>,
    pub dependencies_to_projects_map: HashMap<String, Vec<String>>,
    pub projects_to_dependencies_map: HashMap<String, Vec<String>>,
}

impl Coordinator {
    pub fn new() -> Self {
        Self {
            watchers_target: Vec::new(),
            active_watchers: Vec::new(),
            dependencies_to_projects_map: HashMap::new(),
            projects_to_dependencies_map: HashMap::new(),
        }
    }

    pub fn start(self) -> Result<bool, NodeSpaceError> {
        let shared_logger = create_logging_file()?;

        log_to_file("created logger", &shared_logger)?;

        let pid = std::process::id();
        let pid_manager = CoordinatorPIDManager::new();

        pid_manager.write_pid(pid)?;

        let message = format!("Coordinator started with PID: {}", pid.to_string());
        log_to_file(&message, &shared_logger)?;

        create_socket_file(&shared_logger)?;

        log_to_file("created socket file", &shared_logger)?;
        let listener = init_listener(&shared_logger)?;

        let term = handle_termination_signals()?;

        let (kill_thread_sender_stream, kill_thread_reciever_stream) = std::sync::mpsc::channel();
        let (kill_thread_sender_file_handler, kill_thread_reciever_file_handler) =
            std::sync::mpsc::channel();

        let lock = Arc::new(Mutex::new(self));

        let (reciver, current_watcher) = create_watcher_instance()?;

        log_to_file("created watcher", &shared_logger)?;

        let shared_watcher = Arc::new(Mutex::new(current_watcher));

        let thread_handle_listener = CoordinatorListener::handle_listener(
            listener,
            Arc::clone(&lock),
            kill_thread_reciever_stream,
            Arc::clone(&shared_logger),
        );

        let thread_handle_file_change = CoordinatorUpdatesManager::handle_file_change(
            reciver,
            Arc::clone(&lock),
            kill_thread_reciever_file_handler,
            Arc::clone(&shared_logger),
        );

        CoordinatorWatcherHandler::handle_watcher(
            Arc::clone(&lock),
            Arc::clone(&shared_watcher),
            term,
            Arc::clone(&shared_logger),
        );

        log_to_file("send kill kill_thread_sender_stream", &shared_logger)?;
        send_thread_kill_signal(kill_thread_sender_stream);

        log_to_file("send kill kill_thread_sender_file_handler", &shared_logger)?;
        send_thread_kill_signal(kill_thread_sender_file_handler);

        match thread_handle_listener.join() {
            Ok(_) => {
                log_to_file("joined listener", &shared_logger)?;
            }
            Err(_) => {
                return Err(NodeSpaceError::SocketError(
                    SocketError::ErrorConnectingToSocket("error closing thread".to_string()),
                ));
            }
        };

        match thread_handle_file_change.join() {
            Ok(_) => {
                log_to_file("joined file handle thread", &shared_logger)?;
            }
            Err(_) => {
                return Err(NodeSpaceError::SocketError(
                    SocketError::ErrorConnectingToSocket("error closing thread".to_string()),
                ));
            }
        };

        log_to_file("threads closed", &shared_logger)?;

        Ok(true)
    }
}

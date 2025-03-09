use std::sync::atomic::{AtomicBool, Ordering};
use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use notify::{
    event::{CreateKind, ModifyKind, RemoveKind},
    Error, EventKind, INotifyWatcher, RecursiveMode,
};
use notify_debouncer_full::{DebouncedEvent, Debouncer, NoCache};

use crate::watch_coordinator::coordinator::handle_signals::handle_termination_signals;
use crate::watch_coordinator::coordinator::listener_utils::init_listener;
use crate::watch_coordinator::coordinator::log_utils::{create_logging_file, log_to_file, LogFile};
use crate::watch_coordinator::coordinator::process_stream_request::process_stream_request;
use crate::watch_coordinator::coordinator::socket_file::{create_socket_file, delete_socket_file};
use crate::watch_coordinator::coordinator::thread_utils::send_thread_kill_signal;
use crate::{
    command_line::node_build::run_node_command,
    errors::{node_space::NodeSpaceError, socket::SocketError, watcher::WatcherError},
    watcher_utils::create_watcher_instance,
};

type SharedWatcherClone = Arc<Mutex<Debouncer<INotifyWatcher, NoCache>>>;

pub struct Coordinator {
    pub watchers_target: Vec<String>,
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

    fn handle_request(coordinator_lock: &Arc<Mutex<Self>>, stream: UnixStream) {
        let mut reader = BufReader::new(&stream);
        let mut data_str = String::new();

        if reader.read_line(&mut data_str).is_err() {
            eprintln!("failed to read data from stream");

            return;
        }

        process_stream_request(&coordinator_lock, &data_str);
    }

    fn handle_watcher(
        coordinator_lock: Arc<Mutex<Self>>,
        watcher_lock: SharedWatcherClone,
        term: Arc<AtomicBool>,
        shared_logger: LogFile,
    ) {
        let _ = log_to_file("handle_file_change", &shared_logger);

        loop {
            if term.load(Ordering::Relaxed) {
                let _ = log_to_file("got kill", &shared_logger);

                break;
            }

            let mut current_watcher = match watcher_lock.lock() {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("{}", error.to_string());

                    continue;
                }
            };

            let mut coordinator = match coordinator_lock.lock() {
                Ok(value) => value,
                Err(error) => {
                    eprintln!("{}", error.to_string());

                    continue;
                }
            };

            let current_watcher_targets = coordinator.watchers_target.clone();

            for watcher_target in current_watcher_targets.iter() {
                if !coordinator.active_watchers.contains(watcher_target) {
                    let _ = log_to_file(
                        &format!("adding watcher: {}", watcher_target),
                        &shared_logger,
                    );

                    coordinator.active_watchers.push(watcher_target.to_string());

                    match current_watcher.watch(Path::new(watcher_target), RecursiveMode::Recursive)
                    {
                        Ok(_) => {}
                        Err(error) => {
                            eprintln!("{}", WatcherError::CantCreateWatcher(error.to_string()))
                        }
                    };
                }
            }

            let mut new_list = coordinator.active_watchers.clone();
            let current_list_clone = coordinator.active_watchers.clone();

            for (index, active_watcher) in current_list_clone.iter().enumerate() {
                if !coordinator.watchers_target.contains(active_watcher) {
                    let _ = log_to_file(
                        &format!("removing watcher: {}", &active_watcher),
                        &shared_logger,
                    );

                    new_list.remove(index);

                    match current_watcher.unwatch(Path::new(active_watcher)) {
                        Ok(_) => {}
                        Err(error) => {
                            eprintln!("{}", WatcherError::CantCreateWatcher(error.to_string()))
                        }
                    };
                }
            }

            coordinator.active_watchers = new_list;
        }
    }

    pub fn handle_listener(
        listener: UnixListener,
        coordinator_lock: Arc<Mutex<Self>>,
        kill_thread_reciever: Receiver<String>,
        shared_logger: LogFile,
    ) -> JoinHandle<()> {
        let handler = thread::spawn(move || {
            let _ = log_to_file("handle_listener", &shared_logger);

            loop {
                let kill_massage = kill_thread_reciever.try_recv();

                if let Ok(message) = kill_massage {
                    let _ = log_to_file("Terminating.", &shared_logger);
                    let _ = log_to_file(&message, &shared_logger);

                    break;
                }

                let accept_data = listener.accept();

                if accept_data.is_err() {
                    continue;
                }

                let (stream, _) = accept_data.unwrap();

                Self::handle_request(&coordinator_lock, stream);
            }

            drop(listener);

            delete_socket_file();
        });

        handler
    }

    pub fn handle_change_file_events(
        events: Vec<DebouncedEvent>,
        coordinator_lock: &Arc<Mutex<Self>>,
        shared_logger: &LogFile,
    ) {
        let mut paths: Vec<String> = Vec::new();

        let coordinator = match coordinator_lock.lock() {
            Ok(value) => value,
            Err(error) => {
                eprintln!("{}", error.to_string());

                return;
            }
        };

        for event in events.iter() {
            let event_paths = event
                .paths
                .iter()
                .map(|path| {
                    let path_option = path.to_str();

                    if path_option.is_none() {
                        let _ = log_to_file("couldn't convert path to string", shared_logger);
                        eprintln!("couldn't convert path to string");

                        return "";
                    }

                    path_option.unwrap()
                })
                .filter(|x| *x != "");

            let was_file_changed = match event.kind {
                EventKind::Create(CreateKind::Any) => true,
                EventKind::Modify(ModifyKind::Any) => true,
                EventKind::Remove(RemoveKind::Any) => true,
                _ => false,
            };

            if !was_file_changed {
                continue;
            }

            for current_event_path in event_paths {
                for watched_path in coordinator.watchers_target.iter() {
                    let is_from_project = current_event_path.starts_with(watched_path);

                    if is_from_project && !paths.contains(&watched_path) {
                        paths.push(watched_path.clone());
                    }
                }
            }
        }

        for path in paths {
            let _ = log_to_file(&format!("running build for path: {}", &path), shared_logger);

            match run_node_command(&path, "build") {
                Ok(_) => (),
                Err(error) => {
                    eprintln!("error building project: {}", error);
                }
            };

            // handle_notify_parents(&path);
        }
    }

    pub fn handle_file_change(
        reciver: Receiver<Result<Vec<DebouncedEvent>, Vec<Error>>>,
        coordinator_lock: Arc<Mutex<Self>>,
        kill_thread_reciever: Receiver<String>,
        shared_logger: LogFile,
    ) -> JoinHandle<()> {
        let handle = thread::spawn(move || loop {
            let kill_massage = kill_thread_reciever.try_recv();

            if let Ok(message) = kill_massage {
                dbg!("Terminating.", message);
                break;
            }

            let event_result_msg = reciver.try_recv();

            if event_result_msg.is_err() {
                continue;
            }

            let event_result = event_result_msg.unwrap();

            if event_result.is_err() {
                continue;
            }

            let events = event_result.unwrap();

            Self::handle_change_file_events(events, &coordinator_lock, &shared_logger);
        });

        handle
    }

    pub fn start(self) -> Result<bool, NodeSpaceError> {
        let shared_logger = create_logging_file()?;

        log_to_file("created logger", &shared_logger)?;

        create_socket_file()?;

        log_to_file("created socket file", &shared_logger)?;

        let term = handle_termination_signals()?;
        let (kill_thread_sender_stream, kill_thread_reciever_stream) = std::sync::mpsc::channel();
        let (kill_thread_sender_file_handler, kill_thread_reciever_file_handler) =
            std::sync::mpsc::channel();

        let lock = Arc::new(Mutex::new(self));
        let listener = init_listener(&shared_logger)?;

        let (reciver, current_watcher) = create_watcher_instance()?;
        let shared_watcher = Arc::new(Mutex::new(current_watcher));

        let thread_handle_listener = Self::handle_listener(
            listener,
            Arc::clone(&lock),
            kill_thread_reciever_stream,
            Arc::clone(&shared_logger),
        );

        let thread_handle_file_change = Self::handle_file_change(
            reciver,
            Arc::clone(&lock),
            kill_thread_reciever_file_handler,
            Arc::clone(&shared_logger),
        );

        Self::handle_watcher(
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

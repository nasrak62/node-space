use std::{
    fs,
    sync::{mpsc::Receiver, Arc, Mutex, MutexGuard},
    thread::{self, JoinHandle},
};

use notify::{Error, EventKind};
use notify_debouncer_full::DebouncedEvent;

use crate::{
    command_line::node_build::run_node_command,
    watch_coordinator::coordinator::log_utils::{log_to_file, LogFile},
};

use super::{coordinator::Coordinator, unique_vec::UniqueVec};

pub struct CoordinatorUpdatesManager;

impl CoordinatorUpdatesManager {
    pub fn new() -> Self {
        Self
    }

    fn handle_event(
        paths: &mut UniqueVec<String>,
        event: &DebouncedEvent,
        shared_logger: &LogFile,
        coordinator: &MutexGuard<'_, Coordinator>,
    ) {
        let event_paths = event
            .paths
            .iter()
            .map(|path| {
                let canonical_path = fs::canonicalize(path);

                if canonical_path.is_err() {
                    let path_error = match path.to_str() {
                        Some(value) => value,
                        None => "",
                    };

                    let _ = log_to_file(
                        &format!("couldn't convert path to canonical, {}", path_error),
                        shared_logger,
                    );

                    return "".to_string();
                }

                let canonical_path_buf = canonical_path.unwrap();
                let canonical_path = canonical_path_buf.to_str();

                if canonical_path.is_none() {
                    let _ = log_to_file("couldn't convert path to string", shared_logger);

                    return "".to_string();
                }

                let current_path = canonical_path.unwrap();

                let _ = log_to_file(
                    &format!("got change events: {}", &current_path),
                    shared_logger,
                );

                current_path.to_string()
            })
            .filter(|x| *x != "");

        let was_file_changed = match event.kind {
            EventKind::Create(_) => true,
            EventKind::Modify(_) => true,
            EventKind::Remove(_) => true,
            _ => false,
        };

        if !was_file_changed {
            return;
        }

        let targets = coordinator.watchers_target.clone();

        for current_event_path in event_paths {
            for watched_path in targets.iter() {
                let is_from_project = current_event_path.starts_with(&watched_path.path);

                let output_path = match watched_path.path.ends_with("/") {
                    true => watched_path.path.clone() + &watched_path.output_name,
                    false => watched_path.path.clone() + "/" + &watched_path.output_name,
                };

                let is_output_folder = current_event_path.starts_with(&output_path);

                if !is_from_project {
                    continue;
                }

                if is_output_folder {
                    continue;
                }

                paths.push(watched_path.path.clone());

                // TODO: recursion
                let parents = coordinator
                    .dependencies_to_projects_map
                    .get(&watched_path.path);

                let parents = match parents {
                    Some(value) => value.to_vec(),
                    None => vec![],
                };

                dbg!(
                    &parents,
                    &watched_path.path,
                    &coordinator.dependencies_to_projects_map
                );

                for path in parents {
                    paths.push(path);
                }
            }
        }
    }

    pub fn handle_change_file_events(
        events: Vec<DebouncedEvent>,
        coordinator_lock: &Arc<Mutex<Coordinator>>,
        shared_logger: &LogFile,
    ) {
        let mut paths: UniqueVec<String> = UniqueVec::new();

        let coordinator = match coordinator_lock.lock() {
            Ok(value) => value,
            Err(error) => {
                let _ = log_to_file(
                    &format!("error getting coordinator: {}", error.to_string()),
                    shared_logger,
                );

                return;
            }
        };

        for event in events.iter() {
            Self::handle_event(&mut paths, event, shared_logger, &coordinator)
        }

        let _ = log_to_file(
            &format!(
                "target paths: {}",
                &paths
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<String>>()
                    .join("\n")
            ),
            shared_logger,
        );

        for path in paths {
            let _ = log_to_file(&format!("running build for path: {}", &path), shared_logger);

            match run_node_command(&path, "build") {
                Ok(_) => (),
                Err(error) => {
                    let _ =
                        log_to_file(&format!("error building project: {}", error), shared_logger);
                }
            };
        }
    }

    pub fn handle_file_change(
        reciver: Receiver<Result<Vec<DebouncedEvent>, Vec<Error>>>,
        coordinator_lock: Arc<Mutex<Coordinator>>,
        kill_thread_reciever: Receiver<String>,
        shared_logger: LogFile,
    ) -> JoinHandle<()> {
        let handle = thread::spawn(move || loop {
            let kill_massage = kill_thread_reciever.try_recv();

            if let Ok(message) = kill_massage {
                let _ = log_to_file(&format!("Terminating. {} ", &message), &shared_logger);

                break;
            }

            let event_result_msg = reciver.try_recv();

            if event_result_msg.is_err() {
                continue;
            }

            let event_result = event_result_msg.unwrap();

            if event_result.is_err() {
                let _ = log_to_file("event result error", &shared_logger);

                continue;
            }

            let events = event_result.unwrap();

            Self::handle_change_file_events(events, &coordinator_lock, &shared_logger);
        });

        handle
    }
}

use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc, Mutex,
    },
};

use notify::RecursiveMode;

use crate::{
    errors::watcher::WatcherError,
    watch_coordinator::coordinator::log_utils::{log_to_file, LogFile},
};

use super::coordinator::{Coordinator, SharedWatcherClone};

pub struct CoordinatorWatcherHandler;

impl CoordinatorWatcherHandler {
    pub fn new() -> Self {
        Self
    }

    pub fn handle_watcher(
        coordinator_lock: Arc<Mutex<Coordinator>>,
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
                    let _ = log_to_file(
                        &format!("error getting watcher: {}", error.to_string()),
                        &shared_logger,
                    );

                    continue;
                }
            };

            let mut coordinator = match coordinator_lock.lock() {
                Ok(value) => value,
                Err(error) => {
                    let _ = log_to_file(
                        &format!("error getting coordinator: {}", error.to_string()),
                        &shared_logger,
                    );

                    continue;
                }
            };

            let current_watcher_targets = coordinator.watchers_target.clone();

            for watcher_target in current_watcher_targets.iter() {
                if !coordinator.active_watchers.contains(&watcher_target.path) {
                    let _ = log_to_file(
                        &format!("adding watcher: {}", &watcher_target.path),
                        &shared_logger,
                    );

                    coordinator
                        .active_watchers
                        .push(watcher_target.path.clone());

                    match current_watcher
                        .watch(Path::new(&watcher_target.path), RecursiveMode::Recursive)
                    {
                        Ok(_) => {}
                        Err(error) => {
                            let _ = log_to_file(
                                &format!("{}", WatcherError::CantCreateWatcher(error.to_string())),
                                &shared_logger,
                            );
                        }
                    };
                }
            }

            let mut new_list = coordinator.active_watchers.clone();
            let current_list_clone = coordinator.active_watchers.clone();

            for (index, active_watcher) in current_list_clone.iter().enumerate() {
                if !coordinator
                    .watchers_target
                    .iter()
                    .any(|p| &p.path == active_watcher)
                {
                    let _ = log_to_file(
                        &format!("removing watcher: {}", &active_watcher),
                        &shared_logger,
                    );

                    new_list.remove(index);

                    match current_watcher.unwatch(Path::new(active_watcher)) {
                        Ok(_) => {}
                        Err(error) => {
                            let _ = log_to_file(
                                &format!("{}", WatcherError::CantCreateWatcher(error.to_string())),
                                &shared_logger,
                            );
                        }
                    };
                }
            }

            coordinator.active_watchers = new_list;
        }
    }
}

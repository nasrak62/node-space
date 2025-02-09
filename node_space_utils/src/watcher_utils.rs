use std::{path::Path, time::Duration};

use notify::{EventKind, RecursiveMode};
use notify_debouncer_full::{new_debouncer, DebouncedEvent};

const DEBOUNCE_TIMEOUT: Duration = Duration::from_secs(3);

use crate::{command_line::node_build::run_node_command, errors::watcher::WatcherError};

pub fn extract_should_build_from_event(events: Vec<DebouncedEvent>) -> bool {
    for event in events.iter() {
        let should_build = match event.kind {
            EventKind::Create(_) | EventKind::Modify(_) | EventKind::Remove(_) => true,
            _ => false,
        };

        if should_build {
            return true;
        }
    }

    false
}

// TODO: update coordinator  if needed
// recive update commands from coordinator
pub fn add_watcher(path: &str) -> Result<(), WatcherError> {
    let (sender, reciver) = std::sync::mpsc::channel();

    let mut current_watcher = match new_debouncer(DEBOUNCE_TIMEOUT, None, sender) {
        Ok(value) => value,
        Err(error) => return Err(WatcherError::CantCreateWatcher(error.to_string())),
    };

    match current_watcher.watch(Path::new(path), RecursiveMode::Recursive) {
        Ok(_) => {}
        Err(error) => return Err(WatcherError::CantCreateWatcher(error.to_string())),
    };

    loop {
        let event_option = match reciver.recv() {
            Ok(value) => match value {
                Ok(event_data) => Some(event_data),
                Err(errors) => {
                    for error in errors.iter() {
                        eprintln!("error reciving event: {}", error);
                    }

                    None
                }
            },
            Err(error) => {
                eprintln!("error reciving event: {}", error);

                None
            }
        };

        if event_option.is_none() {
            continue;
        }

        let events = event_option.unwrap();
        let should_build = extract_should_build_from_event(events);

        if !should_build {
            continue;
        }

        match run_node_command(path, "build") {
            Ok(_) => (),
            Err(error) => {
                eprintln!("error building project: {}", error);
            }
        };
    }
}

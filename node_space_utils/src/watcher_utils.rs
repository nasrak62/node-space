use std::{path::Path, time::Duration};

use notify::{EventKind, INotifyWatcher, RecursiveMode};
use notify_debouncer_full::{new_debouncer, DebouncedEvent, Debouncer, NoCache};

const DEBOUNCE_TIMEOUT: Duration = Duration::from_secs(3);

type NotifyReciver = std::sync::mpsc::Receiver<Result<Vec<DebouncedEvent>, Vec<notify::Error>>>;
type Watcher = Debouncer<INotifyWatcher, NoCache>;

use crate::{
    command_line::node_build::run_node_command,
    errors::{node_space::NodeSpaceError, watcher::WatcherError},
    modals::socket_build_data::SocketBuildData,
    watch_coordinator::coordinator_communication::send_data_to_coordinator,
};

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

pub fn create_watcher_instance() -> Result<(NotifyReciver, Watcher), WatcherError> {
    let (sender, reciver) = std::sync::mpsc::channel();

    let current_watcher = match new_debouncer(DEBOUNCE_TIMEOUT, None, sender) {
        Ok(value) => value,
        Err(error) => return Err(WatcherError::CantCreateWatcher(error.to_string())),
    };

    Ok((reciver, current_watcher))
}

pub fn create_watcher(path: &str) -> Result<(NotifyReciver, Watcher), WatcherError> {
    let (reciver, mut current_watcher) = create_watcher_instance()?;

    match current_watcher.watch(Path::new(path), RecursiveMode::Recursive) {
        Ok(_) => {}
        Err(error) => return Err(WatcherError::CantCreateWatcher(error.to_string())),
    };

    Ok((reciver, current_watcher))
}

pub fn create_watcher_loop(path: &str, reciver: NotifyReciver) {
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

// TODO: recive update commands from coordinator
pub fn add_watcher(path: &str) -> Result<(), WatcherError> {
    let (reciver, _watcher) = create_watcher(path)?;

    create_watcher_loop(path, reciver);

    Ok(())
}

pub fn add_local_watcher(data: SocketBuildData) -> Result<(), NodeSpaceError> {
    dbg!("add_local_watcher");

    let path = data.project.path.clone();
    let (reciver, _watcher) = create_watcher(&path)?;

    send_data_to_coordinator(data)?;

    match run_node_command(&path, "build") {
        Ok(_) => (),
        Err(error) => {
            eprintln!("error building project: {}", error);
        }
    };

    create_watcher_loop(&path, reciver);

    Ok(())
}

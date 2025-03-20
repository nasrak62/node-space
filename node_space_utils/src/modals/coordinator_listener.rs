use std::{
    io::{BufRead, BufReader},
    os::unix::net::{UnixListener, UnixStream},
    sync::{mpsc::Receiver, Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::watch_coordinator::coordinator::{
    log_utils::{log_to_file, LogFile},
    process_stream_request::process_stream_request,
    socket_file::delete_socket_file,
};

use super::coordinator::Coordinator;

pub struct CoordinatorListener;

impl CoordinatorListener {
    pub fn new() -> Self {
        Self
    }

    fn handle_request(
        coordinator_lock: &Arc<Mutex<Coordinator>>,
        stream: UnixStream,
        shared_logger: &LogFile,
    ) {
        let mut reader = BufReader::new(&stream);
        let mut data_str = String::new();

        if reader.read_line(&mut data_str).is_err() {
            let _ = log_to_file("failed to read data from stream", &shared_logger);

            return;
        }

        if data_str.is_empty() {
            let _ = log_to_file("got empty data", &shared_logger);

            return;
        }

        process_stream_request(&coordinator_lock, &data_str, shared_logger);
    }

    pub fn handle_listener(
        listener: UnixListener,
        coordinator_lock: Arc<Mutex<Coordinator>>,
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

                Self::handle_request(&coordinator_lock, stream, &shared_logger);
            }

            drop(listener);

            delete_socket_file(&shared_logger);
        });

        handler
    }
}

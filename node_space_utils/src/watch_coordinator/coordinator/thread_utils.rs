use std::sync::mpsc::Sender;

pub fn send_thread_kill_signal(sender: Sender<String>) {
    if let Err(error) = sender.send(String::from("kill")) {
        // remove socket directly
        eprintln!("{}", error);
    };
}

use crossbeam::channel::RecvTimeoutError;
use rwlog::{receiver, sender, Level};
use std::time::Duration;

fn main() {
    let logger_network_sender = sender::Logger::to_network(
        Level::Trace,
        &sender::NetworkSettings {
            local_socket: "127.0.0.1:1234".to_string(),
            destination_socket: "127.0.0.1:2345".to_string(),
        },
    );

    let logger_console = sender::Logger::to_console(Level::Trace);

    let logger_receiver = receiver::Logger::new(Level::Trace, "127.0.0.1:2345");

    rwlog::trace!(&logger_network_sender, "Hello Debug Trace!");
    rwlog::info!(&logger_network_sender, "Hello Debug Info!");
    rwlog::warn!(&logger_network_sender, "Hello Debug Warn!");
    rwlog::err!(&logger_network_sender, "Hello Debug Error!");

    rwlog::rel_trace!(&logger_network_sender, "Hello Trace!");
    rwlog::rel_info!(&logger_network_sender, "Hello Info!");
    rwlog::rel_warn!(&logger_network_sender, "Hello Warn!");
    rwlog::rel_err!(&logger_network_sender, "Hello Error!");

    let mut done = false;
    while !done {
        match logger_receiver.next_message(Some(Duration::from_millis(1000))) {
            Ok(x) => {
                rwlog::rel_info!(&logger_console, "{:?}", x);
            }
            Err(RecvTimeoutError::Disconnected) => {
                done = true;
            }
            Err(RecvTimeoutError::Timeout) => {
                rwlog::rel_info!(&logger_console, "Timeout reached.");
            }
        }
    }
}

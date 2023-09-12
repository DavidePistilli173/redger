use rwlog::{Level, Logger, NetworkSettings};

fn main() {
    let logger = Logger::to_network(
        Level::Trace,
        &NetworkSettings {
            local_socket: "127.0.0.1:1234".to_string(),
            destination_socket: "127.0.0.1:2345".to_string(),
        },
    );
    rwlog::trace!(&logger, "Hello Debug Trace!");
    rwlog::info!(&logger, "Hello Debug Info!");
    rwlog::warn!(&logger, "Hello Debug Warn!");
    rwlog::err!(&logger, "Hello Debug Error!");

    rwlog::rel_trace!(&logger, "Hello Trace!");
    rwlog::rel_info!(&logger, "Hello Info!");
    rwlog::rel_warn!(&logger, "Hello Warn!");
    rwlog::rel_err!(&logger, "Hello Error!");
}

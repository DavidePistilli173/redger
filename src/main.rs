use rwlog::{Level, Logger, Target};

fn main() {
    let logger = Logger::new(Level::Trace, Target::Console);
    rwlog::trace!(&logger, "Hello Debug Trace!");
    rwlog::info!(&logger, "Hello Debug Info!");
    rwlog::warn!(&logger, "Hello Debug Warn!");
    rwlog::err!(&logger, "Hello Debug Error!");
    rwlog::fatal!(&logger, "Hello Debug Fatal!");

    rwlog::rel_trace!(&logger, "Hello Trace!");
    rwlog::rel_info!(&logger, "Hello Info!");
    rwlog::rel_warn!(&logger, "Hello Warn!");
    rwlog::rel_err!(&logger, "Hello Error!");
    rwlog::rel_fatal!(&logger, "Hello Fatal!");
}

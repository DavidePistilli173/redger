fn main() {
    let logger = rwlog::sender::Logger::to_console(rwlog::Level::Trace);
    pollster::block_on(rwgfx::run(logger.clone()));
}

mod app_data;

use rwui::app::{App, WindowAppDescriptor};

use app_data::AppData;

fn main() {
    // Initialise the logger.
    let logger = rwlog::sender::Logger::to_console(rwlog::Level::Trace);

    rwlog::trace!(
        &logger,
        "Executable path: {}",
        std::env::current_dir().unwrap().display()
    );

    let app_data = AppData::new(logger.clone());

    let app_descriptor = WindowAppDescriptor {
        logger: logger.clone(),
        user_app: app_data,
    };

    let (event_loop, mut window_app) = App::new(app_descriptor).unwrap_or_else(|e| {
        rwlog::fatal!(&logger, "Failed to create the main window: {e}.");
        std::process::exit(1);
    });

    window_app.init();
    event_loop.run_app(&mut window_app).unwrap_or_else(|e| {
        rwlog::fatal!(&logger, "Fatal error in the main loop: {e}.");
        std::process::exit(1);
    });
}

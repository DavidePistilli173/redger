use rwgfx::context::Context;
use rwgfx::error::RenderError;
use winit::event::WindowEvent;
use winit::event_loop::EventLoop;
use winit::window::{Window, WindowBuilder};

fn main() {
    // Initialise the logger.
    let logger = rwlog::sender::Logger::to_console(rwlog::Level::Trace);

    // Create a new event loop.
    let event_loop = EventLoop::new();

    // Create the window.
    let window = WindowBuilder::new().build(&event_loop).unwrap_or_else(|e| {
        rwlog::rel_fatal!(&logger, "Failed to create window: {e}.");
    });

    let context = rwgfx::context::Context::new(
        logger.clone(),
        &window,
        window.inner_size().width,
        window.inner_size().height,
    )
    .unwrap_or_else(|e| {
        rwlog::rel_fatal!(&logger, "Failed to create application: {e}.");
    });
    run(logger, window, event_loop, context);
}

/// Run the main loop of the application.
fn run(logger: rwlog::sender::Logger, window: Window, event_loop: EventLoop, mut context: Context) {
    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Process incoming events.
        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } => {
                if window_id == window.id() && !app.propagate_event(&event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            app.resize(physical_size.width, physical_size.height)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            app.resize(*new_inner_size.width, *new_inner_size.height)
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                match app.render() {
                    Ok(_) => (),
                    Err(RenderError::SurfaceInvalid) => {
                        app.resize(window.inner_size().width, window.inner_size().height)
                    }
                    Err(RenderError::OutOfMemory) => {
                        rwlog::rel_err!(&logger, "Not enough GPU memory!");
                        *control_flow = ControlFlow::Exit;
                    }
                    Err(RenderError::GraphicsDeviceNotResponding) => {
                        rwlog::warn!(&logger, "{e}");
                    }
                };
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }

        // Update the application.
        context.update();
    });
}

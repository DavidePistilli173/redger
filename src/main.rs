use cgmath::{Point2, Vector2};
use rwgfx::context::Context;
use rwgfx::error::RenderError;
use rwui::button::Button;
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
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

    let button = Button::new(
        &context,
        Point2::<f32> { x: 350.0, y: 250.0 },
        Vector2::<f32> { x: 100.0, y: 100.0 },
        -75.0,
        [0.5, 0.05, 0.05, 1.0],
    );

    run(logger, window, event_loop, context, button);
}

/// Run the main loop of the application.
fn run(
    logger: rwlog::sender::Logger,
    window: Window,
    event_loop: EventLoop<()>,
    mut context: Context,
    mut button: Button,
) {
    let mut last_update_time = chrono::Local::now();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Process incoming events.
        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } => {
                if window_id == window.id() && !button.consume_event(&event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            context.resize(physical_size.width, physical_size.height)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            context.resize(new_inner_size.width, new_inner_size.height)
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                match context.render(|queue, render_pass, _| button.draw(queue, render_pass)) {
                    Ok(_) => (),
                    Err(RenderError::SurfaceInvalid) => {
                        context.resize(window.inner_size().width, window.inner_size().height)
                    }
                    Err(RenderError::OutOfMemory) => {
                        rwlog::rel_err!(&logger, "Not enough GPU memory!");
                        *control_flow = ControlFlow::Exit;
                    }
                    Err(RenderError::GraphicsDeviceNotResponding) => {
                        rwlog::warn!(&logger, "Graphics device not responding.");
                    }
                };
            }
            Event::MainEventsCleared => {
                window.request_redraw();
            }
            _ => (),
        }

        // Update the application.
        let current_time = chrono::Local::now();
        let delta_time = current_time - last_update_time;
        last_update_time = current_time;
        button.update(&delta_time);
    });
}

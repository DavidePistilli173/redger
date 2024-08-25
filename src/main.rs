use cgmath::{Point2, Vector2};
use rwgfx::asset;
use rwgfx::error::RenderError;
use rwgfx::renderer::Renderer;
use rwui::button::{Button, ButtonDescriptor};
use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::{Window, WindowBuilder};

struct UiCtx {
    toggle: bool,
}

fn main() {
    // Initialise the logger.
    let logger = rwlog::sender::Logger::to_console(rwlog::Level::Trace);

    rwlog::trace!(
        &logger,
        "Executable path: {}",
        std::env::current_dir().unwrap().display()
    );

    // Create a new event loop.
    let event_loop = EventLoop::new();

    // Create the window.
    let window = WindowBuilder::new()
        .with_title("REDGER")
        .build(&event_loop)
        .unwrap_or_else(|e| {
            rwlog::fatal!(&logger, "Failed to create window: {e}.");
            std::process::exit(1);
        });

    let renderer = Renderer::new(
        logger.clone(),
        &window,
        window.inner_size().width,
        window.inner_size().height,
    )
    .unwrap_or_else(|e| {
        rwlog::fatal!(&logger, "Failed to create application: {e}.");
        std::process::exit(1);
    });

    let mut asset_manager = asset::Manager::new_with_defaults(&logger, renderer.ctx())
        .unwrap_or_else(|err| {
            rwlog::fatal!(&logger, "Failed to create the asset manager: {err}.");
            std::process::exit(1);
        });

    let button = Button::new(
        &logger,
        &renderer,
        &mut asset_manager,
        &ButtonDescriptor {
            position: Point2::<f32> { x: 350.0, y: 250.0 },
            size: Vector2::<f32> { x: 100.0, y: 100.0 },
            z_index: -75.0,
            back_colour: [0.05, 0.05, 0.05, 1.0],
            texture_id: Some(rwgfx::texture::ID_HAMBURGER),
            label: Some("PROVA".to_string()),
            on_press: None,
            on_release: Some(|button: &mut Button<UiCtx>, data: &mut UiCtx| {
                if data.toggle {
                    button.set_position_offset(Vector2::<f32> { x: 50.0, y: 50.0 });
                    button.set_size_offset(Vector2::<f32> {
                        x: -100.0,
                        y: -100.0,
                    });
                } else {
                    button.set_position_offset(Vector2::<f32> { x: -50.0, y: -50.0 });
                    button.set_size_offset(Vector2::<f32> { x: 100.0, y: 100.0 });
                }
                data.toggle = !data.toggle;
            }),
            on_enter: None,
            on_exit: None,
        },
    );

    run(logger, window, event_loop, renderer, asset_manager, button);
}

/// Run the main loop of the application.
fn run(
    logger: rwlog::sender::Logger,
    window: Window,
    event_loop: EventLoop<()>,
    mut renderer: Renderer,
    mut asset_manager: asset::Manager,
    mut button: Button<UiCtx>,
) {
    let mut last_update_time = chrono::Local::now();
    let mut uictx = UiCtx { toggle: false };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        // Process incoming events.
        match event {
            Event::WindowEvent {
                window_id,
                ref event,
            } => {
                if window_id == window.id() && !button.on_event(&mut uictx, &event) {
                    match event {
                        WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            renderer.resize(physical_size.width, physical_size.height)
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            renderer.resize(new_inner_size.width, new_inner_size.height)
                        }
                        _ => (),
                    }
                }
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                if let Err(err) = renderer.draw(&mut asset_manager, &[&button]) {
                    match err {
                        RenderError::SurfaceInvalid => {
                            renderer.resize(window.inner_size().width, window.inner_size().height)
                        }
                        RenderError::OutOfMemory => {
                            rwlog::err!(&logger, "Not enough GPU memory!");
                            *control_flow = ControlFlow::Exit;
                        }
                        RenderError::GraphicsDeviceNotResponding => {
                            rwlog::warn!(&logger, "Graphics device not responding.");
                        }
                    }
                }
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

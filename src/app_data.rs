use cgmath::Vector2;
use rwgfx::render_interface::RenderInterface;
use rwgfx::{mesh::MeshDescriptor, renderer, vertex::Vertex};
use rwlog::sender::Logger;
use rwui::app::App;
use rwui::app_interface::AppInterface;
use rwui::button::{Button, ButtonDescriptor};
use rwui::user_app::UserApp;
use rwui::WindowEvent;

/// States of the application.
pub enum AppState {
    /// Initialisation state.
    Init,
    /// Running state.
    Running,
}

pub struct ButtonData {
    /// Logger.
    logger: Logger,
    /// Test flag.
    flag: bool,
}

/// Application data.
pub struct AppData {
    /// Logger.
    logger: Logger,
    /// Current state of the application.
    state: AppState,
    /// Test button.
    button: Option<Button<ButtonData>>,
    /// Data for the button to operate on.
    button_data: ButtonData,
    /// Interface to the host application.
    app_interface: Option<AppInterface>,
    /// Interface to the window renderer.
    render_interface: Option<RenderInterface>,
}

impl AppData {
    /// Create a new app data object.
    pub fn new(logger: Logger) -> AppData {
        AppData {
            logger: logger.clone(),
            state: AppState::Init,
            button: None,
            button_data: ButtonData {
                logger,
                flag: false,
            },
            app_interface: None,
            render_interface: None,
        }
    }
}

impl UserApp for AppData {
    /// Function called when the application is initialised.
    /// Must return true on success and false otherwise.
    fn on_init(&mut self, app_interface: AppInterface, render_interface: RenderInterface) -> bool {
        let button = Button::new(ButtonDescriptor {
            render_interface: render_interface.clone(),
            position: cgmath::Point2 { x: -0.5, y: -0.5 },
            size: cgmath::Vector2 { x: 1.0, y: 1.0 },
            z_index: 1.0,
            back_colour: [1.0, 0.0, 0.0, 1.0],
            on_press: None,
            on_release: Some(button_on_release),
            on_enter: None,
            on_exit: None,
        });

        if let Ok(b) = button {
            self.button = Some(b);
            self.state = AppState::Running;
            return true;
        }

        false
    }

    /// Function called when a new event has arrived.
    fn on_event(&mut self, event: &WindowEvent) {
        self.button
            .as_mut()
            .unwrap()
            .on_event(&mut self.button_data, event);
        //rwlog::trace!(&self.logger, "On event {:?}!", event);
    }

    /// Function called before drawing each frame.
    fn on_draw(&mut self) {
        //rwlog::trace!(&self.logger, "On draw!");
    }

    fn on_update(&mut self, elapsed: &chrono::Duration) {
        if self.button.is_some() {
            self.button.as_mut().unwrap().update(elapsed);
        }
    }
}

fn button_on_release(button: &mut Button<ButtonData>, data: &mut ButtonData) {
    rwlog::info!(&data.logger, "Button on release!");

    if data.flag {
        button.set_position_offset(Vector2::<f32> { x: 0.25, y: 0.25 });
        button.set_size_offset(Vector2::<f32> { x: 0.5, y: 0.5 });
    } else {
        button.set_position_offset(Vector2::<f32> { x: -0.25, y: -0.25 });
        button.set_size_offset(Vector2::<f32> { x: -0.5, y: -0.5 });
    }
    data.flag = !data.flag;
}

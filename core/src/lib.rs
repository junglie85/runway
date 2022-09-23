use platform::Hal;
use thiserror::Error;
use tracing::info;
use winit::{
    event::{Event, WindowEvent},
    event_loop::ControlFlow,
};

pub use crate::config::{load_config, Config};
pub use crate::log::init_logging;
pub use crate::platform::init_platform;

mod config;
mod log;
mod platform;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("initialization error")]
    Init(#[source] Box<dyn std::error::Error + Send>),
    #[error("i/o error")]
    Io(#[source] Box<dyn std::error::Error + Send>),
    #[error("platform error")]
    Platform(#[source] Box<dyn std::error::Error + Send>),
}

impl EngineError {
    fn init(e: impl std::error::Error + Send + 'static) -> Self {
        Self::Init(Box::new(e))
    }

    fn io(e: impl std::error::Error + Send + 'static) -> Self {
        Self::Io(Box::new(e))
    }

    fn platform(e: impl std::error::Error + Send + 'static) -> Self {
        Self::Platform(Box::new(e))
    }
}

struct Engine {}

impl Engine {
    fn new() -> Result<Self, EngineError> {
        info!("Engine::new");

        Ok(Self {})
    }

    fn tick(&mut self) {
        info!("Engine::tick");
    }

    fn shutdown(&mut self) {
        info!("Engine::shutdown");
    }

    fn close_requested(&self) -> bool {
        false
    }
}

impl Drop for Engine {
    fn drop(&mut self) {
        self.shutdown();
    }
}

pub fn launch(_config: &Config, hal: Hal) -> Result<(), EngineError> {
    let event_loop = hal.event_loop;
    let window = hal.window;

    let mut e = Engine::new()?;

    event_loop.run(move |event, _, control_flow| {
        match event {
            Event::MainEventsCleared {} => {
                // Timing.
                e.tick();

                window.request_redraw();
            }
            Event::RedrawRequested(window_id) if window_id == window.id() => {
                // Render
            }
            Event::WindowEvent { event, window_id } if window_id == window.id() => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                _ => (),
            },
            _ if e.close_requested() => *control_flow = ControlFlow::Exit,
            _ => *control_flow = ControlFlow::Poll,
        }
    });
}

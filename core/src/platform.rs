use winit::{
    event_loop::EventLoop,
    window::{Window, WindowBuilder},
};

use crate::{Config, EngineError};

pub struct Hal {
    pub(crate) event_loop: EventLoop<()>,
    pub(crate) window: Window,
}

pub fn init_platform(config: &Config) -> Result<Hal, EngineError> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(&format!("{} - {}", config.game.title, config.version))
        .build(&event_loop)
        .map_err(EngineError::platform)?;

    Ok(Hal { event_loop, window })
}

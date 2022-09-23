use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;
use thiserror::Error;
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

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

#[derive(Debug, Deserialize)]
struct Config {
    version: String,
    game: GameConfig,
}

#[derive(Debug, Deserialize)]
struct GameConfig {
    title: String,
}

struct Engine {}

impl Engine {
    fn new() -> Result<Self, EngineError> {
        println!("Engine::new");

        Ok(Self {})
    }

    fn tick(&mut self) {
        println!("Engine::tick");
    }

    fn shutdown(&mut self) {
        println!("Engine::shutdown");
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

fn load_config() -> Result<Config, EngineError> {
    let cwd = std::env::var("CARGO_MANIFEST_DIR").map_err(EngineError::io)?;
    let path = [&cwd, "config", "sandbox.toml"].iter().collect::<PathBuf>();
    let file = read_to_string(&path).map_err(EngineError::init)?;
    let config = toml::from_str::<Config>(&file).map_err(EngineError::init)?;

    Ok(config)
}

fn init_platform(config: &Config) -> Result<(EventLoop<()>, Window), EngineError> {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title(&format!("{} - {}", config.game.title, config.version))
        .build(&event_loop)
        .map_err(EngineError::platform)?;

    Ok((event_loop, window))
}

pub fn launch() -> Result<(), EngineError> {
    println!("launch");

    let config = load_config()?;
    let (event_loop, window) = init_platform(&config)?;

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

use std::{fs::read_to_string, path::PathBuf};

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum EngineError {
    #[error("initialization error")]
    Init(#[source] Box<dyn std::error::Error + Send>),
    #[error("i/o error")]
    Io(#[source] Box<dyn std::error::Error + Send>),
}

impl EngineError {
    fn init(e: impl std::error::Error + Send + 'static) -> Self {
        Self::Init(Box::new(e))
    }

    fn io(e: impl std::error::Error + Send + 'static) -> Self {
        Self::Io(Box::new(e))
    }
}

#[derive(Debug, Deserialize, Serialize)]
struct EngineConfig {
    ticks: i32,
}

struct Engine {
    count: i32,
    ticks: i32,
}

impl Engine {
    fn new() -> Self {
        println!("Engine::new");

        Self { count: 0, ticks: 0 }
    }

    fn init(&mut self) -> Result<(), EngineError> {
        println!("Engine::init");

        let cwd = std::env::var("CARGO_MANIFEST_DIR").map_err(EngineError::io)?;
        let path = [&cwd, "config", "sandbox.ron"].iter().collect::<PathBuf>();
        let file = read_to_string(&path).map_err(EngineError::init)?;
        let config = ron::from_str::<EngineConfig>(&file).map_err(EngineError::init)?;

        self.ticks = config.ticks;

        Ok(())
    }

    fn tick(&mut self) {
        println!("Engine::tick - {}", self.count);

        self.count += 1;
    }

    fn shutdown(&mut self) {
        println!("Engine::shutdown");
    }

    fn close_requested(&self) -> bool {
        self.count >= self.ticks
    }
}

fn init_platform() -> Result<(), EngineError> {
    println!("init_platform");

    Ok(())
}

fn launch() -> Result<(), EngineError> {
    println!("launch");

    let mut e = Engine::new();
    e.init()?;
    while !e.close_requested() {
        e.tick();
    }
    e.shutdown();

    Ok(())
}

pub fn engine_main() -> Result<(), EngineError> {
    init_platform()?;
    launch()?;

    Ok(())
}

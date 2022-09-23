use std::{fs::read_to_string, path::PathBuf};

use serde::Deserialize;

use crate::EngineError;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub version: String,
    pub game: GameConfig,
}

#[derive(Debug, Deserialize)]
pub struct GameConfig {
    pub title: String,
}

pub fn load_config() -> Result<Config, EngineError> {
    let cwd = std::env::var("CARGO_MANIFEST_DIR").map_err(EngineError::io)?;
    let path = [&cwd, "config", "config.toml"].iter().collect::<PathBuf>();
    let file = read_to_string(&path).map_err(EngineError::init)?;
    let config = toml::from_str::<Config>(&file).map_err(EngineError::init)?;

    Ok(config)
}

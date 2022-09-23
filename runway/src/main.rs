use core::{init_logging, init_platform, launch, load_config, EngineError};

fn main() -> Result<(), EngineError> {
    let config = load_config()?;
    init_logging(&config)?;
    let hal = init_platform(&config)?;

    launch(&config, hal)?;

    Ok(())
}

use engine::engine_main;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    engine_main()?;

    Ok(())
}

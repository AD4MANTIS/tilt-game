use color_eyre::eyre::Result;
use tilt_game::run;

fn main() -> Result<()> {
    color_eyre::install()?;

    Ok(run()?)
}

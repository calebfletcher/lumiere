use std::error::Error;

use lumiere::ppm;

fn main() -> Result<(), Box<dyn Error>> {
    ppm::write_image(256, 256)?;

    Ok(())
}

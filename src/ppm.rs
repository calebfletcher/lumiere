use std::fs::File;
use std::io::{self, Write};

use crate::{colour, Colour};

pub fn write_image(width: usize, height: usize) -> Result<(), io::Error> {
    let max_value: u64 = 255;

    // Create the file
    let mut file = File::create("image.ppm")?;

    // Print image header
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", width, height).as_bytes())?;
    file.write_all(format!("{}\n", max_value).as_bytes())?;

    for row in 0..height {
        // Print progress indicator
        eprint!("\rScanlines remaining: {} ", height - row);
        io::stderr().flush()?;

        for col in 0..width {
            // Calculate RGB components
            let pixel = Colour {
                x: col as f64 / (width - 1) as f64,
                y: row as f64 / (width - 1) as f64,
                z: 0.25,
            };

            colour::write_colour(&file, pixel)?;
        }
    }
    eprintln!("\nDone");

    Ok(())
}

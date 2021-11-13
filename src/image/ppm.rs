use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn write_image<P: AsRef<Path>, const WIDTH: usize, const HEIGHT: usize>(
    pixels: [[[u8; 3]; WIDTH]; HEIGHT],
    path: P,
) -> Result<(), io::Error> {
    let max_value: u64 = 255;

    // Create the file
    let mut file = File::create(path)?;

    // Print image header
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", WIDTH, HEIGHT).as_bytes())?;
    file.write_all(format!("{}\n", max_value).as_bytes())?;

    for row in 0..HEIGHT {
        for col in 0..WIDTH {
            let pixel = pixels[row][col];
            file.write_all(format!("{} {} {}\n", pixel[0], pixel[1], pixel[2]).as_bytes())?
        }
    }

    Ok(())
}

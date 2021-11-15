use std::fs::File;
use std::io::{self, Write};
use std::path::Path;

pub fn write_image<P: AsRef<Path>, const WIDTH: usize, const HEIGHT: usize>(
    pixels: &[u8; 3 * WIDTH * HEIGHT],
    path: P,
) -> Result<(), io::Error>
where
    [(); WIDTH * HEIGHT * 3]: Sized,
{
    let max_value: u64 = 255;

    // Create the file
    let mut file = File::create(path)?;

    // Print image header
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", WIDTH, HEIGHT).as_bytes())?;
    file.write_all(format!("{}\n", max_value).as_bytes())?;

    for pixel in pixels.chunks_exact(3) {
        file.write_all(format!("{} {} {}\n", pixel[0], pixel[1], pixel[2]).as_bytes())?
    }

    Ok(())
}

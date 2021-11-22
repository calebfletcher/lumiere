use std::fs::File;
use std::io::{self, BufWriter};
use std::path::Path;

pub fn write_image<P: AsRef<Path>, const WIDTH: usize, const HEIGHT: usize>(
    pixels: &[u8],
    path: P,
) -> Result<(), io::Error> {
    // Create the file
    let file = File::create(path)?;
    let w = BufWriter::new(file);

    // Create the PNG encoder
    let mut encoder = png::Encoder::new(
        w,
        WIDTH.try_into().expect("Width cannot be larger than u32"),
        HEIGHT.try_into().expect("Height cannot be larger than u32"),
    );
    encoder.set_color(png::ColorType::Rgb);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header()?;

    // Save image to disk
    writer.write_image_data(pixels)?;

    Ok(())
}

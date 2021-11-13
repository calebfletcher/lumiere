use std::fs::File;
use std::io::{self, BufWriter};
use std::mem;
use std::path::Path;

pub fn write_image<P: AsRef<Path>, const WIDTH: usize, const HEIGHT: usize>(
    pixels: [[[u8; 3]; WIDTH]; HEIGHT],
    path: P,
) -> Result<(), io::Error>
where
    [(); WIDTH * HEIGHT * 3]: Sized,
{
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

    // This is safe as the underlying memory layout of the two types is
    // identical, just one is nested while the other is flattened.
    let pixel_buffer: &[u8; WIDTH * HEIGHT * 3] = unsafe { mem::transmute(&pixels) };

    // Save image to disk
    writer.write_image_data(pixel_buffer)?;

    Ok(())
}

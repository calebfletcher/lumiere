use std::{fs::File, path::Path};

use crate::{Colour, Point3};

use super::Texture;

#[derive(Debug)]
pub struct ImageTexture {
    buf: Vec<u8>,
    info: png::OutputInfo,
}

impl ImageTexture {
    pub fn new<P>(path: P) -> Self
    where
        P: AsRef<Path>,
    {
        let decoder = png::Decoder::new(File::open(path).unwrap());
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).unwrap();
        buf.truncate(info.buffer_size());
        Self { buf, info }
    }
}

impl Texture for ImageTexture {
    fn get_value(&self, u: f64, v: f64, _p: &Point3) -> Colour {
        let u = u.clamp(0., 1.);
        let v = 1. - v.clamp(0., 1.);

        let i = (u * self.info.width as f64) as usize;
        let j = (v * self.info.height as f64) as usize;

        let bytes_per_pixel = self.info.color_type.samples();

        let pixel_offset = j * self.info.width as usize + i;
        let pixel = &self.buf[bytes_per_pixel * pixel_offset..bytes_per_pixel * (pixel_offset + 1)];
        let colour_scale = 1.0 / 255.0;
        Colour::new(
            pixel[0] as f64 * colour_scale,
            pixel[1] as f64 * colour_scale,
            pixel[2] as f64 * colour_scale,
        )
    }
}

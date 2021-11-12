use std::io::{self, Write};

pub fn write_colour(mut stream: impl Write, colour: crate::Colour) -> io::Result<()> {
    let ir = (colour.x * 255.999) as u64;
    let ig = (colour.y * 255.999) as u64;
    let ib = (colour.z * 255.999) as u64;

    // Write pixel to file
    stream.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())
}

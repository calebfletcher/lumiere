use std::{
    error::Error,
    fs::File,
    io::{self, Write},
};

use lumiere::{colour, ray::Ray, ray_colour, vec3::Vec3, Point3};

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    let aspect_ratio = 16. / 9.;
    let image_width = 400 as usize;
    let image_height = (image_width as f64 / aspect_ratio) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    let max_value: u64 = 255;

    // Create the file
    let mut file = File::create("image.ppm")?;

    // Print image header
    file.write_all(b"P3\n")?;
    file.write_all(format!("{} {}\n", image_width, image_height).as_bytes())?;
    file.write_all(format!("{}\n", max_value).as_bytes())?;

    for row in 0..image_height {
        // Print progress indicator
        eprint!("\rScanlines remaining: {} ", image_height - row);
        io::stderr().flush()?;

        for col in 0..image_width {
            let u = col as f64 / (image_width - 1) as f64;
            let v = row as f64 / (image_height - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_colour = ray_colour(&r);

            colour::write_colour(&file, pixel_colour)?;
        }
    }
    eprintln!("\nDone");

    Ok(())
}

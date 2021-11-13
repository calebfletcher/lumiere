use std::{
    error::Error,
    io::{self, Write},
    path::Path,
};

use lumiere::{image, ray::Ray, ray_colour, vec3::Vec3, Point3};

fn main() -> Result<(), Box<dyn Error>> {
    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0., 0., 0.);
    let horizontal = Vec3::new(viewport_width, 0., 0.);
    let vertical = Vec3::new(0., viewport_height, 0.);
    let lower_left_corner =
        origin - horizontal / 2. - vertical / 2. - Vec3::new(0., 0., focal_length);

    // Pixel array as height * rows * channels 8 bit values
    let mut pixels = [[[0_u8; 3]; IMAGE_WIDTH]; IMAGE_HEIGHT];

    #[allow(clippy::needless_range_loop)]
    for row in 0..IMAGE_HEIGHT {
        // Print progress indicator
        eprint!("\rScanlines remaining: {} ", IMAGE_HEIGHT - row);
        io::stderr().flush()?;

        for col in 0..IMAGE_WIDTH {
            let u = col as f64 / (IMAGE_WIDTH - 1) as f64;
            let v = row as f64 / (IMAGE_HEIGHT - 1) as f64;
            let r = Ray::new(
                origin,
                lower_left_corner + horizontal * u + vertical * v - origin,
            );
            let pixel_colour = ray_colour(&r);

            pixels[row][col][0] = (pixel_colour.x * 255.) as u8;
            pixels[row][col][1] = (pixel_colour.y * 255.) as u8;
            pixels[row][col][2] = (pixel_colour.z * 255.) as u8;
        }
    }
    eprintln!("\nRaytracing Completed");

    image::png::write_image(pixels, Path::new("image.png"))?;
    eprintln!("Saved image");

    Ok(())
}

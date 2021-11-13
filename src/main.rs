use std::{
    error::Error,
    io::{self, Write},
    path::Path,
};

use rand::Rng;

use lumiere::{camera, image, object, ray_colour, Colour, Point3};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel = 100;

    // World
    let mut world = object::HittableList::new();
    world.add(Box::new(object::Sphere::new(Point3::new(0., 0., -1.), 0.5)));
    world.add(Box::new(object::Sphere::new(
        Point3::new(0., -100.5, 0.),
        100.,
    )));

    // Camera
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(0., 0., 0.))
        .focal_length(1.)
        .build();

    // Pixel array as height * rows * channels 8 bit values
    let mut pixels = [[[0_u8; 3]; IMAGE_WIDTH]; IMAGE_HEIGHT];

    #[allow(clippy::needless_range_loop)]
    for row in 0..IMAGE_HEIGHT {
        // Print progress indicator
        eprint!("\rScanlines remaining: {} ", IMAGE_HEIGHT - row);
        io::stderr().flush()?;

        for col in 0..IMAGE_WIDTH {
            let mut pixel_colour = Colour::zeros();
            for _ in 0..samples_per_pixel {
                let u = (col as f64 + rng.gen::<f64>()) / (IMAGE_WIDTH - 1) as f64;
                let v = (row as f64 + rng.gen::<f64>()) / (IMAGE_HEIGHT - 1) as f64;
                let r = camera.get_ray(u, v);
                pixel_colour += ray_colour(&r, &world);
            }
            pixel_colour /= samples_per_pixel as f64;

            pixels[row][col][0] = (pixel_colour.x * 255.999) as u8;
            pixels[row][col][1] = (pixel_colour.y * 255.999) as u8;
            pixels[row][col][2] = (pixel_colour.z * 255.999) as u8;
        }
    }
    eprintln!("\nRaytracing Completed");

    image::png::write_image(pixels, Path::new("image.png"))?;
    eprintln!("Saved image");

    Ok(())
}

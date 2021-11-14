use std::{
    error::Error,
    io::{self, Write},
    path::Path,
};

use rand::Rng;

use lumiere::{camera, image, material, object, ray_colour, Colour, Point3};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 900;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel: usize = 100;
    let max_depth = 50;

    let material_ground = Box::new(material::Lambertian::new(Colour::new(0.8, 0.8, 0.0)));
    let material_centre = Box::new(material::Dielectric::new(1.5));
    let material_left = Box::new(material::Dielectric::new(1.5));
    let material_right = Box::new(material::Metal::new(Colour::new(0.8, 0.6, 0.2), 1.0));

    // World
    let mut world = object::HittableList::new();

    // Objects
    world.add(Box::new(object::Sphere::new(
        String::from("surface"),
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(object::Sphere::new(
        String::from("glass ball"),
        Point3::new(0., 0., -1.),
        0.5,
        material_centre,
    )));
    world.add(Box::new(object::Sphere::new(
        String::from("ball other"),
        Point3::new(-1., 0., -1.),
        0.5,
        material_left,
    )));
    world.add(Box::new(object::Sphere::new(
        String::from("rear ball"),
        Point3::new(5., 0., -10.),
        3.5,
        material_right,
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
                pixel_colour += ray_colour(&r, &world, max_depth, &mut rng);
            }
            pixel_colour /= samples_per_pixel as f64;

            pixels[row][col][0] = (pixel_colour.x.sqrt() * 255.999) as u8;
            pixels[row][col][1] = (pixel_colour.y.sqrt() * 255.999) as u8;
            pixels[row][col][2] = (pixel_colour.z.sqrt() * 255.999) as u8;
        }
    }
    eprintln!("\nRaytracing Completed");

    image::png::write_image(pixels, Path::new("image.png"))?;
    eprintln!("Saved image");

    Ok(())
}

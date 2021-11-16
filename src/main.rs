use std::{error::Error, path::Path};

use lumiere::{camera, image, material, object, scene::Scene, vec3::Vec3, Colour, Point3};
use rand::Rng;

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 900;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel: usize = 100;
    let max_depth = 50;

    // Camera
    let camera_look_dir = Point3::new(-13., -2., -3.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(13., 2., 3.))
        .look_dir(camera_look_dir)
        .fov(20.)
        .aperture(0.1)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    // Ground
    let material_ground = Box::new(material::Lambertian::new(Colour::new(0.5, 0.5, 0.5)));
    world.add(Box::new(object::Sphere::new(
        "ground".to_string(),
        Point3::new(0., -1000., 0.),
        1000.,
        material_ground,
    )));

    // Random small balls
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat: f64 = rng.gen();
            let centre = Point3::new(
                a as f64 + 0.9 * choose_mat,
                0.2,
                b as f64 + 0.9 * choose_mat,
            );

            if (centre - Point3::new(4., 0.2, 0.)).length() > 0.9 {
                match choose_mat {
                    a if a < 0.8 => {
                        // diffuse
                        let albedo = Colour::random_in_range_inclusive(&mut rng, 0.0..=1.0);
                        let sphere_material = material::Lambertian::new(albedo);
                        let centre_1 = centre + Vec3::new(0., rng.gen_range(0.0..=0.5), 0.);
                        world.add(Box::new(object::MovingSphere::new(
                            "".to_string(),
                            centre,
                            centre_1,
                            0.2,
                            Box::new(sphere_material),
                        )));
                    }
                    a if a < 0.95 => {
                        // metal
                        let albedo = Colour::random_in_range_inclusive(&mut rng, 0.5..=1.0);
                        let fuzziness: f64 = rng.gen_range(0.0..0.5);
                        let sphere_material = material::Metal::new(albedo, fuzziness);
                        world.add(Box::new(object::Sphere::new(
                            "".to_string(),
                            centre,
                            0.2,
                            Box::new(sphere_material),
                        )));
                    }
                    _ => {
                        // glass
                        let sphere_material = material::Dielectric::new(1.5);
                        world.add(Box::new(object::Sphere::new(
                            "".to_string(),
                            centre,
                            0.2,
                            Box::new(sphere_material),
                        )));
                    }
                }
            }
        }
    }

    let material_1 = Box::new(material::Dielectric::new(1.5));
    world.add(Box::new(object::Sphere::new(
        "obj1".to_string(),
        Point3::new(0., 1., 0.),
        1.,
        material_1,
    )));

    let material_2 = Box::new(material::Lambertian::new(Colour::new(0.4, 0.2, 0.1)));
    world.add(Box::new(object::Sphere::new(
        "obj2".to_string(),
        Point3::new(-4., 1., 0.),
        1.,
        material_2,
    )));

    let material_3 = Box::new(material::Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(object::Sphere::new(
        "obj3".to_string(),
        Point3::new(4., 1., 0.),
        1.,
        material_3,
    )));

    // Pixel array as height * rows * channels 8 bit values
    const BUFFER_LENGTH: usize = 3 * IMAGE_WIDTH * IMAGE_HEIGHT;
    let mut pixels = vec![0_u8; BUFFER_LENGTH];

    let scene = Scene::new(
        world,
        camera,
        max_depth,
        samples_per_pixel,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
    );

    scene.render(&mut pixels, &mut rng)?;

    image::png::write_image::<&Path, IMAGE_WIDTH, IMAGE_HEIGHT>(&pixels, Path::new("image.png"))?;
    eprintln!("Saved image");

    Ok(())
}

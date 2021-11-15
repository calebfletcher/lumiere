use std::{error::Error, path::Path};

use lumiere::{camera, image, material, object, scene::Scene, Colour, Point3};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rand::thread_rng();

    // Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 600;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel: usize = 100;
    let max_depth = 50;

    // Camera
    let camera_look_dir = Point3::new(-3., -3., -3.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(3., 3., 2.))
        .look_dir(camera_look_dir)
        .fov(40.)
        .aperture(0.3)
        .focus_dist(camera_look_dir.length())
        .build();

    // World
    let mut world = object::HittableList::new();

    // Materials
    let material_ground = Box::new(material::Lambertian::new(Colour::new(0.8, 0.8, 0.)));
    let material_centre = Box::new(material::Lambertian::new(Colour::new(0.1, 0.2, 0.5)));
    let material_left = Box::new(material::Dielectric::new(1.5));
    let material_right = Box::new(material::Metal::new(Colour::new(0.8, 0.6, 0.2), 0.));

    // Objects
    world.add(Box::new(object::Sphere::new(
        String::from("ball other"),
        Point3::new(0., -100.5, -1.),
        100.,
        material_ground,
    )));
    world.add(Box::new(object::Sphere::new(
        String::from("rear ball"),
        Point3::new(0., 0., -1.),
        0.5,
        material_centre,
    )));
    world.add(Box::new(object::Sphere::new(
        String::from("ball other1"),
        Point3::new(-1., 0., -1.),
        0.5,
        material_left.clone(),
    )));
    world.add(Box::new(object::Sphere::new(
        String::from("ball other2"),
        Point3::new(-1., 0., -1.),
        -0.4,
        material_left,
    )));
    world.add(Box::new(object::Sphere::new(
        String::from("rear ball"),
        Point3::new(1., 0., -1.),
        0.5,
        material_right,
    )));

    // Pixel array as height * rows * channels 8 bit values
    const BUFFER_LENGTH: usize = 3 * IMAGE_WIDTH * IMAGE_HEIGHT;
    let mut pixels = [0_u8; BUFFER_LENGTH];

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

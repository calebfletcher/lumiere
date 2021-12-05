use std::{error::Error, path::Path, sync::Arc};

use rand::{rngs, SeedableRng};

use lumiere::{bvh::BVHNode, camera, image, material, object, scene::Scene, Colour, Point3};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rngs::SmallRng::from_rng(rand::thread_rng()).unwrap();

    // Image parameters
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 1024;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel: usize = 5000;
    let max_depth = 50;

    // Pixel array as height * rows * channels 8 bit values
    const BUFFER_LENGTH: usize = 3 * IMAGE_WIDTH * IMAGE_HEIGHT;
    let mut pixels = vec![0_u8; BUFFER_LENGTH];

    // Generate the objects

    // Camera
    let camera_look_dir = Point3::new(-13., -2., -3.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(13., 2., 3.))
        .look_dir(camera_look_dir)
        .fov(20.)
        .aspect_ratio(ASPECT_RATIO)
        .aperture(0.1)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    // Ground
    let material_ground = Arc::new(material::Lambertian::from_colour(Colour::new(
        0.5, 0.5, 0.5,
    )));
    world.add(Arc::new(object::Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        material_ground,
    )));

    let material_3 = Arc::new(material::Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(object::Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material_3,
    )));

    let material_4 = Arc::new(material::Lambertian::from_colour(Colour::new(
        0.7, 0.1, 0.5,
    )));
    world.add(Arc::new(object::Sphere::new(
        Point3::new(7., 1., 0.),
        0.4,
        material_4,
    )));

    // Generate BVH tree
    let mut bvh_root = object::HittableList::new();
    bvh_root.add(Arc::new(BVHNode::new(world, &mut rng)));

    // Create scene
    let scene = Scene::new(
        bvh_root,
        camera,
        max_depth,
        samples_per_pixel,
        IMAGE_WIDTH,
        IMAGE_HEIGHT,
        Colour::new(0.7, 0.8, 1.),
    );

    // Render the scene to a frame buffer
    scene.render(&mut pixels)?;

    // Write the frame buffer to a file
    image::png::write_image::<&Path, IMAGE_WIDTH, IMAGE_HEIGHT>(&pixels, Path::new("image.png"))?;
    eprintln!("Saved image");

    Ok(())
}

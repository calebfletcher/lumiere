use std::{error::Error, path::Path, sync::Arc};

use lumiere::{
    bvh::BVHNode, camera, image, material, object, scene::Scene, texture, Colour, Point3,
};
use rand::{rngs, SeedableRng};

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
        .aperture(0.)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    // Sphere 1
    let checker = Arc::new(texture::CheckerTexture::from_colours(
        0.8,
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let material_1 = Arc::new(material::Lambertian::new(checker.clone()));
    world.add(Arc::new(object::Sphere::new(
        Point3::new(0., -10., 0.),
        10.,
        material_1,
    )));

    // Sphere 2
    let material_2 = Arc::new(material::Lambertian::new(checker));
    world.add(Arc::new(object::Sphere::new(
        Point3::new(0., 10., 0.),
        10.,
        material_2,
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

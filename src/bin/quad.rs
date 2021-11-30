use std::{error::Error, path::Path, rc::Rc};

use lumiere::{
    bvh::BVHNode, camera, image, material, object, scene::Scene, vec3::Vec3, Colour, Point3,
};

use rand::{rngs, SeedableRng};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rngs::SmallRng::from_entropy();

    // Image parameters
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 400;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel: usize = 1000;
    let max_depth = 50;

    // Pixel array as height * rows * channels 8 bit values
    const BUFFER_LENGTH: usize = 3 * IMAGE_WIDTH * IMAGE_HEIGHT;
    let mut pixels = vec![0_u8; BUFFER_LENGTH];

    // Generate the objects

    // Camera
    let camera_look_dir = Point3::new(0., 0., -9.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(0., 0., 9.))
        .look_dir(camera_look_dir)
        .fov(80.)
        .aspect_ratio(ASPECT_RATIO)
        .aperture(0.)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    let left_red = Rc::new(material::Lambertian::from_colour(Colour::new(1., 0.2, 0.2)));
    let back_green = Rc::new(material::Lambertian::from_colour(Colour::new(0.2, 1., 0.2)));
    let right_blue = Rc::new(material::Lambertian::from_colour(Colour::new(0.2, 0.2, 1.)));
    let upper_orange = Rc::new(material::Lambertian::from_colour(Colour::new(1., 0.5, 0.)));
    let lower_teal = Rc::new(material::Lambertian::from_colour(Colour::new(
        0.2, 0.8, 0.8,
    )));

    world.add(Box::new(object::Quad::new(
        Point3::new(-3., -2., 5.),
        Vec3::new(0., 0., -4.),
        Vec3::new(0., 4., 0.),
        left_red,
    )));
    world.add(Box::new(object::Quad::new(
        Point3::new(-2., -2., 0.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 4., 0.),
        back_green,
    )));
    world.add(Box::new(object::Quad::new(
        Point3::new(3., -2., 1.),
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        right_blue,
    )));
    world.add(Box::new(object::Quad::new(
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        upper_orange,
    )));
    world.add(Box::new(object::Quad::new(
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        lower_teal,
    )));

    // Generate BVH tree
    let mut bvh_root = object::HittableList::new();
    bvh_root.add(Box::new(BVHNode::new(world, &mut rng)));

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
    scene.render(&mut pixels, &mut rng)?;

    // Write the frame buffer to a file
    image::png::write_image::<&Path, IMAGE_WIDTH, IMAGE_HEIGHT>(&pixels, Path::new("image.png"))?;
    eprintln!("Saved image");

    Ok(())
}

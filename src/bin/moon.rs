use std::{error::Error, path::Path, rc::Rc};

use lumiere::{
    bvh::BVHNode, camera, image, material, object, scene::Scene, texture, Colour, Point3,
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
    let camera_look_dir = Point3::new(-26., -1., -6.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(26., 3., 6.))
        .look_dir(camera_look_dir)
        .fov(20.)
        .aspect_ratio(ASPECT_RATIO)
        .aperture(0.)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    let noise = Rc::new(texture::NoiseTexture::new());
    let noise_texture = Rc::new(material::Lambertian::new(noise.clone()));
    world.add(Box::new(object::Sphere::new(
        Point3::new(0., -1000., 0.),
        1000.,
        noise_texture,
    )));

    let noise_texture = Rc::new(material::Lambertian::new(noise));
    world.add(Box::new(object::Sphere::new(
        Point3::new(0., 2., 0.),
        2.,
        noise_texture,
    )));

    let diff_light = Rc::new(material::DiffuseLight::from_colour(Colour::new(4., 4., 4.)));
    world.add(Box::new(object::Quad::new(
        Point3::new(3., 1., -2.),
        Point3::new(2., 0., 0.),
        Point3::new(0., 2., 0.),
        diff_light,
    )));

    let diff_light = Rc::new(material::DiffuseLight::from_colour(Colour::new(4., 4., 4.)));
    world.add(Box::new(object::Sphere::new(
        Point3::new(0., 7., 0.),
        2.,
        diff_light,
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
        Colour::new(0., 0., 0.),
    );

    // Render the scene to a frame buffer
    scene.render(&mut pixels, &mut rng)?;

    // Write the frame buffer to a file
    image::png::write_image::<&Path, IMAGE_WIDTH, IMAGE_HEIGHT>(&pixels, Path::new("image.png"))?;
    eprintln!("Saved image");

    Ok(())
}

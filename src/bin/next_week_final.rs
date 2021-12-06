use std::{error::Error, path::Path, sync::Arc};

use lumiere::{
    bvh::BVHNode, camera, image, material, object, scene::Scene, vec3::Vec3, Colour, Point3,
};

use rand::{rngs, Rng, SeedableRng};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rngs::SmallRng::from_rng(rand::thread_rng()).unwrap();

    // Image parameters
    const ASPECT_RATIO: f64 = 9. / 9.;
    const IMAGE_WIDTH: usize = 900;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel: usize = 3000;
    let max_depth = 4;

    // Pixel array as height * rows * channels 8 bit values
    const BUFFER_LENGTH: usize = 3 * IMAGE_WIDTH * IMAGE_HEIGHT;
    let mut pixels = vec![0_u8; BUFFER_LENGTH];

    // Generate the objects

    // Camera
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(478., 278., -600.))
        .look_at(Point3::new(278., 278., 0.))
        .fov(40.)
        .aspect_ratio(ASPECT_RATIO)
        .aperture(0.)
        .build();

    // World
    let mut world = object::HittableList::new();

    let ground = Arc::new(material::Lambertian::from_colour(Colour::new(
        0.48, 0.83, 0.53,
    )));

    // Ground boxes
    let mut boxes1 = object::HittableList::new();
    let boxes_per_side = 10;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.;
            let x0 = 680. - i as f64 * w;
            let z0 = -350. + j as f64 * w;
            let y0 = 0.;
            let x1 = x0 + w;
            let y1 = rng.gen_range(1.0..=100.0);
            let z1 = z0 + w;

            boxes1.add(Arc::new(object::quad::new_box(
                &Vec3::new(x0, y0, z0),
                &Vec3::new(x1, y1, z1),
                ground.clone(),
            )))
        }
    }
    world.add(Arc::new(BVHNode::new(boxes1, &mut rng)));
    //world.add(Arc::new(boxes1));

    let light = Arc::new(material::DiffuseLight::from_colour(Colour::new(7., 7., 7.)));
    world.add(Arc::new(object::Quad::new(
        Point3::new(123., 554., 147.),
        Vec3::new(300., 0., 0.),
        Vec3::new(0., 0., 265.),
        light,
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
        Colour::new(0., 0., 0.),
    );

    // Render the scene to a frame buffer
    scene.render(&mut pixels)?;

    // Write the frame buffer to a file
    image::png::write_image::<&Path, IMAGE_WIDTH, IMAGE_HEIGHT>(&pixels, Path::new("image.png"))?;
    eprintln!("Saved image");

    Ok(())
}

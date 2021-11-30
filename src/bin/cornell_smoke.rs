use std::{error::Error, path::Path, rc::Rc};

use lumiere::{
    bvh::BVHNode,
    camera, image, material,
    object::{self, rotate::RotateY, Translate},
    scene::Scene,
    vec3::Vec3,
    Colour, Point3,
};

use rand::{rngs, SeedableRng};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rngs::SmallRng::from_entropy();

    // Image parameters
    const ASPECT_RATIO: f64 = 9. / 9.;
    const IMAGE_WIDTH: usize = 500;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel: usize = 500;
    let max_depth = 50;

    // Pixel array as height * rows * channels 8 bit values
    const BUFFER_LENGTH: usize = 3 * IMAGE_WIDTH * IMAGE_HEIGHT;
    let mut pixels = vec![0_u8; BUFFER_LENGTH];

    // Generate the objects

    // Camera
    let camera_look_dir = Point3::new(0., 0., 1.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(278., 278., -800.))
        .look_dir(camera_look_dir)
        .fov(40.)
        .aspect_ratio(ASPECT_RATIO)
        .aperture(0.)
        .build();

    // World
    let mut world = object::HittableList::new();

    let red = Rc::new(material::Lambertian::from_colour(Colour::new(
        0.65, 0.05, 0.05,
    )));
    let white = Rc::new(material::Lambertian::from_colour(Colour::new(
        0.73, 0.73, 0.73,
    )));
    let green = Rc::new(material::Lambertian::from_colour(Colour::new(
        0.12, 0.45, 0.12,
    )));
    let light = Rc::new(material::DiffuseLight::from_colour(Colour::new(
        15., 15., 15.,
    )));

    world.add(Box::new(object::Quad::new(
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        green,
    )));
    world.add(Box::new(object::Quad::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        red,
    )));
    world.add(Box::new(object::Quad::new(
        Vec3::new(113., 554., 127.),
        Vec3::new(330., 0., 0.),
        Vec3::new(0., 0., 305.),
        light,
    )));
    world.add(Box::new(object::Quad::new(
        Vec3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white.clone(),
    )));
    world.add(Box::new(object::Quad::new(
        Vec3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        white.clone(),
    )));
    world.add(Box::new(object::Quad::new(
        Vec3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        white.clone(),
    )));

    let box1 = Rc::new(object::quad::new_box(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 330., 165.),
        white.clone(),
    ));
    let box1 = Rc::new(RotateY::new(box1, 15.));
    let box1 = Rc::new(Translate::new(box1, Vec3::new(265., 0., 295.)));
    let box1 = Box::new(object::ConstantMedium::from_colour(
        box1,
        0.01,
        Colour::new(0., 0., 0.),
    ));
    world.add(box1);

    let box2 = Rc::new(object::quad::new_box(
        &Point3::new(0., 0., 0.),
        &Point3::new(165., 165., 165.),
        white.clone(),
    ));
    let box2 = Rc::new(RotateY::new(box2, -18.));
    let box2 = Rc::new(Translate::new(box2, Vec3::new(130., 0., 65.)));
    let box2 = Box::new(object::ConstantMedium::from_colour(
        box2,
        0.01,
        Colour::new(1., 1., 1.),
    ));
    world.add(box2);

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

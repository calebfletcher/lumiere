use std::{error::Error, path::Path, rc::Rc};

use lumiere::{
    bvh::BVHNode, camera, image, material, object, scene::Scene, texture, Colour, Point3,
};

use rand::{rngs, Rng, SeedableRng};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rngs::SmallRng::from_entropy();

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
    let checker = Rc::new(texture::CheckerTexture::from_colours(
        0.32,
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Rc::new(material::Lambertian::new(checker));
    world.add(Box::new(object::Sphere::new(
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
                        let sphere_material = Rc::new(material::Lambertian::from_colour(albedo));
                        let centre_1 = centre; // + Vec3::new(0., rng.gen_range(0.0..=0.5), 0.);
                        world.add(Box::new(object::MovingSphere::new(
                            centre,
                            centre_1,
                            0.2,
                            sphere_material,
                        )));
                    }
                    a if a < 0.95 => {
                        // metal
                        let albedo = Colour::random_in_range_inclusive(&mut rng, 0.5..=1.0);
                        let fuzziness: f64 = rng.gen_range(0.0..0.5);
                        let sphere_material = Rc::new(material::Metal::new(albedo, fuzziness));
                        world.add(Box::new(object::Sphere::new(centre, 0.2, sphere_material)));
                    }
                    _ => {
                        // glass
                        let sphere_material = Rc::new(material::Dielectric::new(1.5));
                        world.add(Box::new(object::Sphere::new(centre, 0.2, sphere_material)));
                    }
                }
            }
        }
    }

    let material_1 = Rc::new(material::Dielectric::new(1.5));
    world.add(Box::new(object::Sphere::new(
        Point3::new(0., 1., 0.),
        1.,
        material_1,
    )));

    let material_2 = Rc::new(material::Lambertian::from_colour(Colour::new(
        0.4, 0.2, 0.1,
    )));
    world.add(Box::new(object::Sphere::new(
        Point3::new(-4., 1., 0.),
        1.,
        material_2,
    )));

    let material_3 = Rc::new(material::Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(object::Sphere::new(
        Point3::new(4., 1., 0.),
        1.,
        material_3,
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

use std::{error::Error, path::Path};

use lumiere::{
    bvh::BVHNode, camera, image, material, object, scene::Scene, texture, vec3::Vec3, Colour,
    Point3,
};
use rand::{rngs, Rng, SeedableRng};

fn main() -> Result<(), Box<dyn Error>> {
    let mut rng = rngs::SmallRng::from_entropy();

    // Image parameters
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: usize = 600;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    let samples_per_pixel: usize = 3000;
    let max_depth = 50;

    // Pixel array as height * rows * channels 8 bit values
    const BUFFER_LENGTH: usize = 3 * IMAGE_WIDTH * IMAGE_HEIGHT;
    let mut pixels = vec![0_u8; BUFFER_LENGTH];

    // Generate the objects
    let (camera, world) = example_cornell_box(&mut rng);

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

pub fn example_earth_scene(_rng: &mut rngs::SmallRng) -> (camera::Camera, object::HittableList) {
    // Camera
    let camera_look_dir = Point3::new(0., 0., -12.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(0., 0., 12.))
        .look_dir(camera_look_dir)
        .fov(30.)
        .aperture(0.1)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    let material_4 = Box::new(material::Lambertian::new(Box::new(
        texture::ImageTexture::new("earthmap.png"),
    )));
    world.add(Box::new(object::Sphere::new(
        "obj4".to_string(),
        Point3::new(0., 0., 0.),
        2.,
        material_4,
    )));

    (camera, world)
}

pub fn example_basic_scene(_rng: &mut rngs::SmallRng) -> (camera::Camera, object::HittableList) {
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
    let material_ground = Box::new(material::Lambertian::from_colour(Colour::new(
        0.5, 0.5, 0.5,
    )));
    world.add(Box::new(object::Sphere::new(
        "ground".to_string(),
        Point3::new(0., -1000., 0.),
        1000.,
        material_ground,
    )));

    let material_3 = Box::new(material::Metal::new(Colour::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(object::Sphere::new(
        "obj3".to_string(),
        Point3::new(4., 1., 0.),
        1.,
        material_3,
    )));

    let material_4 = Box::new(material::Lambertian::from_colour(Colour::new(
        0.7, 0.1, 0.5,
    )));
    world.add(Box::new(object::Sphere::new(
        "obj4".to_string(),
        Point3::new(7., 1., 0.),
        0.4,
        material_4,
    )));

    (camera, world)
}

pub fn example_two_spheres_scene(
    _rng: &mut rngs::SmallRng,
) -> (camera::Camera, object::HittableList) {
    // Camera
    let camera_look_dir = Point3::new(-13., -2., -3.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(13., 2., 3.))
        .look_dir(camera_look_dir)
        .fov(20.)
        .aperture(0.)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    // Sphere 1
    let checker_1 = Box::new(texture::CheckerTexture::from_colours(
        0.8,
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let material_1 = Box::new(material::Lambertian::new(checker_1));
    world.add(Box::new(object::Sphere::new(
        "sphere_1".to_string(),
        Point3::new(0., -10., 0.),
        10.,
        material_1,
    )));

    // Sphere 2
    let checker_2 = Box::new(texture::CheckerTexture::from_colours(
        0.8,
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let material_2 = Box::new(material::Lambertian::new(checker_2));
    world.add(Box::new(object::Sphere::new(
        "sphere_1".to_string(),
        Point3::new(0., 10., 0.),
        10.,
        material_2,
    )));

    (camera, world)
}

pub fn example_two_perlin_spheres_scene(
    _rng: &mut rngs::SmallRng,
) -> (camera::Camera, object::HittableList) {
    // Camera
    let camera_look_dir = Point3::new(-13., -2., -3.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(13., 2., 3.))
        .look_dir(camera_look_dir)
        .fov(20.)
        .aperture(0.)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    // Sphere 1
    let texture_1 = Box::new(texture::NoiseTexture::new());
    let material_1 = Box::new(material::Lambertian::new(texture_1));
    world.add(Box::new(object::Sphere::new(
        "sphere_1".to_string(),
        Point3::new(0., -1000., 0.),
        1000.,
        material_1,
    )));

    // Sphere 2
    let texture_2 = Box::new(texture::NoiseTexture::new());
    let material_2 = Box::new(material::Lambertian::new(texture_2));
    world.add(Box::new(object::Sphere::new(
        "sphere_1".to_string(),
        Point3::new(0., 2., 0.),
        2.,
        material_2,
    )));

    (camera, world)
}

pub fn example_complex_scene(rng: &mut rngs::SmallRng) -> (camera::Camera, object::HittableList) {
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
    let checker = Box::new(texture::CheckerTexture::from_colours(
        0.32,
        Colour::new(0.2, 0.3, 0.1),
        Colour::new(0.9, 0.9, 0.9),
    ));
    let material_ground = Box::new(material::Lambertian::new(checker));
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
                        let albedo = Colour::random_in_range_inclusive(rng, 0.0..=1.0);
                        let sphere_material = material::Lambertian::from_colour(albedo);
                        let centre_1 = centre; // + Vec3::new(0., rng.gen_range(0.0..=0.5), 0.);
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
                        let albedo = Colour::random_in_range_inclusive(rng, 0.5..=1.0);
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

    let material_2 = Box::new(material::Lambertian::from_colour(Colour::new(
        0.4, 0.2, 0.1,
    )));
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

    (camera, world)
}

pub fn example_quads_scene(_rng: &mut rngs::SmallRng) -> (camera::Camera, object::HittableList) {
    // Camera
    let camera_look_dir = Point3::new(0., 0., -9.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(0., 0., 9.))
        .look_dir(camera_look_dir)
        .fov(80.)
        .aperture(0.)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    let left_red = Box::new(material::Lambertian::from_colour(Colour::new(1., 0.2, 0.2)));
    let back_green = Box::new(material::Lambertian::from_colour(Colour::new(0.2, 1., 0.2)));
    let right_blue = Box::new(material::Lambertian::from_colour(Colour::new(0.2, 0.2, 1.)));
    let upper_orange = Box::new(material::Lambertian::from_colour(Colour::new(1., 0.5, 0.)));
    let lower_teal = Box::new(material::Lambertian::from_colour(Colour::new(
        0.2, 0.8, 0.8,
    )));

    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Point3::new(-3., -2., 5.),
        Vec3::new(0., 0., -4.),
        Vec3::new(0., 4., 0.),
        left_red,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Point3::new(-2., -2., 0.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 4., 0.),
        back_green,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Point3::new(3., -2., 1.),
        Vec3::new(0., 0., 4.),
        Vec3::new(0., 4., 0.),
        right_blue,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Point3::new(-2., 3., 1.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., 4.),
        upper_orange,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Point3::new(-2., -3., 5.),
        Vec3::new(4., 0., 0.),
        Vec3::new(0., 0., -4.),
        lower_teal,
    )));

    (camera, world)
}

pub fn example_simple_light_scene(
    _rng: &mut rngs::SmallRng,
) -> (camera::Camera, object::HittableList) {
    // Camera
    let camera_look_dir = Point3::new(-26., -1., -6.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(26., 3., 6.))
        .look_dir(camera_look_dir)
        .fov(20.)
        .aperture(0.)
        .focus_dist(10.)
        .build();

    // World
    let mut world = object::HittableList::new();

    let noise = Box::new(texture::NoiseTexture::new());
    let noise_texture = Box::new(material::Lambertian::new(noise));
    world.add(Box::new(object::Sphere::new(
        "".to_string(),
        Point3::new(0., -1000., 0.),
        1000.,
        noise_texture,
    )));

    let noise = Box::new(texture::NoiseTexture::new());
    let noise_texture = Box::new(material::Lambertian::new(noise));
    world.add(Box::new(object::Sphere::new(
        "".to_string(),
        Point3::new(0., 2., 0.),
        2.,
        noise_texture,
    )));

    let diff_light = Box::new(material::DiffuseLight::from_colour(Colour::new(4., 4., 4.)));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Point3::new(3., 1., -2.),
        Point3::new(2., 0., 0.),
        Point3::new(0., 2., 0.),
        diff_light,
    )));

    let diff_light = Box::new(material::DiffuseLight::from_colour(Colour::new(4., 4., 4.)));
    world.add(Box::new(object::Sphere::new(
        "".to_string(),
        Point3::new(0., 7., 0.),
        2.,
        diff_light,
    )));

    (camera, world)
}

pub fn example_cornell_box(_rng: &mut rngs::SmallRng) -> (camera::Camera, object::HittableList) {
    // Camera
    let camera_look_dir = Point3::new(0., 0., 1.);
    let camera = camera::CameraBuilder::new()
        .origin(Point3::new(278., 278., -800.))
        .look_dir(camera_look_dir)
        .fov(40.)
        .aperture(0.)
        .build();

    // World
    let mut world = object::HittableList::new();

    let red = Box::new(material::Lambertian::from_colour(Colour::new(
        0.65, 0.05, 0.05,
    )));
    let white1 = Box::new(material::Lambertian::from_colour(Colour::new(
        0.73, 0.73, 0.73,
    )));
    let white2 = Box::new(material::Lambertian::from_colour(Colour::new(
        0.73, 0.73, 0.73,
    )));
    let white3 = Box::new(material::Lambertian::from_colour(Colour::new(
        0.73, 0.73, 0.73,
    )));
    let green = Box::new(material::Lambertian::from_colour(Colour::new(
        0.12, 0.45, 0.12,
    )));
    let light = Box::new(material::DiffuseLight::from_colour(Colour::new(
        15., 15., 15.,
    )));

    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        green,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Vec3::new(0., 0., 0.),
        Vec3::new(0., 555., 0.),
        Vec3::new(0., 0., 555.),
        red,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Vec3::new(343., 554., 332.),
        Vec3::new(-130., 0., 0.),
        Vec3::new(0., 0., -105.),
        light,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Vec3::new(0., 0., 0.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 0., 555.),
        white1,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Vec3::new(555., 555., 555.),
        Vec3::new(-555., 0., 0.),
        Vec3::new(0., 0., -555.),
        white2,
    )));
    world.add(Box::new(object::Quad::new(
        "".to_string(),
        Vec3::new(0., 0., 555.),
        Vec3::new(555., 0., 0.),
        Vec3::new(0., 555., 0.),
        white3,
    )));

    (camera, world)
}

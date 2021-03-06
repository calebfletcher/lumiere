use std::sync::Arc;

use lumiere::{
    interval, material,
    object::{self, Hittable},
    ray::Ray,
    vec3::Vec3,
    Point3,
};
use rand::{rngs, SeedableRng};

fn main() {
    let mut rng = rngs::SmallRng::from_rng(rand::thread_rng()).unwrap();
    let material_centre = Arc::new(material::Dielectric::new(1.5));

    let sphere = object::Sphere::new(Point3::new(2., 0., 0.), 0.5, material_centre);

    let origin = Vec3::new(0., -0.3, 0.);
    let direction = Vec3::new(1., 0., 0.);
    let ray = Ray::new(origin, direction, 0.);

    println!("iter 1");
    let hitrec = sphere
        .hit(
            &ray,
            &interval::Interval::new(0.0001, f64::INFINITY),
            &mut rng,
        )
        .unwrap();
    dbg!(&hitrec);

    let scatter_result = hitrec.mat.scatter(&ray, &hitrec, &mut rng);
    dbg!(&scatter_result);

    println!("iter 2");
    let hitrec = sphere
        .hit(
            &scatter_result.scattered,
            &interval::Interval::new(0.0001, f64::INFINITY),
            &mut rng,
        )
        .unwrap();
    dbg!(&hitrec);

    let scatter_result = hitrec
        .mat
        .scatter(&scatter_result.scattered, &hitrec, &mut rng);
    dbg!(&scatter_result);
}

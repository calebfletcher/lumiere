#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use object::Hittable;
use ray::Ray;

pub mod camera;
pub mod image;
pub mod object;
pub mod ray;
pub mod vec3;

pub type Point3 = vec3::Vec3;
pub type Colour = vec3::Vec3;

pub fn ray_colour(
    r: &Ray,
    world: &impl Hittable,
    depth: usize,
    rng: &mut impl rand::Rng,
) -> Colour {
    if depth <= 0 {
        return Colour::new(0., 0., 0.);
    }
    match world.hit(r, 0.001, f64::INFINITY) {
        Some(hitrec) => {
            // Ray intersects object
            let target =
                hitrec.point + hitrec.normal + vec3::Vec3::random_in_unit_sphere(rng).unit();
            let new_ray = Ray::new(hitrec.point, target - hitrec.point);
            ray_colour(&new_ray, world, depth - 1, rng) * 0.5
        }
        None => {
            // Ray doesn't intersect any objects
            let unit = r.direction.unit();
            let t = 0.5 * (unit.y + 1.0);
            Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
        }
    }
}

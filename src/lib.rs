#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use material::Behaviour;
use rand::rngs::ThreadRng;

use object::Hittable;
use ray::Ray;

pub mod camera;
pub mod image;
pub mod material;
pub mod object;
pub mod ray;
pub mod vec3;

pub type Point3 = vec3::Vec3;
pub type Colour = vec3::Vec3;

pub fn ray_colour(r: &Ray, world: &impl Hittable, depth: usize, rng: &mut ThreadRng) -> Colour {
    if depth <= 0 {
        return Colour::new(0., 0., 0.);
    }
    match world.hit(r, 0.001, f64::INFINITY) {
        Some(hitrec) => {
            // Ray intersects object

            let scatter_result = hitrec.mat.scatter(r, &hitrec, rng);
            match scatter_result.behaviour {
                Behaviour::Scatter => {
                    scatter_result.attenuation
                        * ray_colour(&scatter_result.scattered, world, depth - 1, rng)
                        * 0.5
                }
                Behaviour::Absorb => Colour::new(0., 0., 0.),
            }
        }
        None => {
            // Ray doesn't intersect any objects
            let unit = r.direction.unit();
            let t = 0.5 * (unit.y + 1.0);
            Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
        }
    }
}

#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use ray::Ray;

pub mod colour;
pub mod image;
pub mod ray;
pub mod vec3;

pub type Point3 = vec3::Vec3;
pub type Colour = vec3::Vec3;

pub fn ray_colour(r: &Ray) -> Colour {
    if hit_sphere(&Point3::new(0., 0., -1.), 0.5, r) {
        return Colour::new(1., 0., 0.);
    }

    let unit = r.direction.unit();
    let t = 0.5 * (unit.y + 1.0);
    Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    let oc = r.origin - *center;
    let a = r.direction.dot(r.direction);
    let b = 2. * oc.dot(r.direction);
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4. * a * c;
    discriminant > 0.
}

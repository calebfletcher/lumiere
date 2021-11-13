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
    let t = hit_sphere(&Point3::new(0., 0., -1.), 0.5, r);

    match t {
        Some(t) => {
            // Ray intersect sphere
            let n = (r.at(t) - Point3::new(0., 0., -1.)).unit();
            Colour::new(n.x + 1., n.y + 1., n.z + 1.) * 0.5
        }
        None => {
            // Ray doesn't intersect sphere
            let unit = r.direction.unit();
            let t = 0.5 * (unit.y + 1.0);
            Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
        }
    }
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> Option<f64> {
    let oc = r.origin - *center;
    let a = r.direction.dot(r.direction);
    let b = 2. * oc.dot(r.direction);
    let c = oc.dot(oc) - radius.powi(2);
    let discriminant = b.powi(2) - 4. * a * c;
    if discriminant < 0. {
        None
    } else {
        Some((-b - discriminant.sqrt()) / (2. * a))
    }
}

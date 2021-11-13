use crate::{ray::Ray, Point3};

use super::object;

pub struct Sphere {
    centre: Point3,
    radius: f64,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        Self { centre, radius }
    }
}

impl object::Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<object::HitRecord> {
        let oc = r.origin - self.centre;
        let a = r.direction.length_squared();
        let half_b = oc.dot(r.direction);
        let c = oc.length_squared() - self.radius.powi(2);
        let discriminant = half_b.powi(2) - a * c;

        // Check if no intersection
        if discriminant < 0. {
            return None;
        }

        // Check if intersection(s) are within [t_min, t_max]
        let root = (-half_b - discriminant.sqrt()) / a;
        if root < t_min || root > t_max {
            let root = (-half_b + discriminant.sqrt()) / a;
            if root < t_min || root > t_max {
                return None;
            }
        }

        let intersection = r.at(root);
        let outward_normal = (intersection - self.centre) / self.radius;
        let mut hitrec = object::HitRecord::new(intersection, outward_normal, root);
        hitrec.set_face_normal(&r, outward_normal);
        Some(hitrec)
    }
}

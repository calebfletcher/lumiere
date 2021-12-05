use std::{f64::consts, sync::Arc};

use rand::rngs;

use crate::{aabb::AABB, interval, material, ray::Ray, vec3::Vec3, Point3};

use super::object;

#[derive(Debug)]
pub struct Sphere {
    centre: Point3,
    radius: f64,
    mat: Arc<dyn material::Material>,
    aabb: AABB,
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64, mat: Arc<dyn material::Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let aabb = AABB::from_points(centre - rvec, centre + rvec);
        Self {
            centre,
            radius,
            mat,
            aabb,
        }
    }

    /// Calculates the u-v coordinate of a point on a sphere.
    /// p: a given point on the sphere of radius one, centered at the origin.
    /// u: returned value [0,1] of angle around the Y axis from X=-1.
    /// v: returned value [0,1] of angle from Y=-1 to Y=+1.
    /// <1 0 0> yields <0.50 0.50>       <-1  0  0> yields <0.00 0.50>
    /// <0 1 0> yields <0.50 1.00>       < 0 -1  0> yields <0.50 0.00>
    /// <0 0 1> yields <0.25 0.50>       < 0  0 -1> yields <0.75 0.50>
    fn get_uv(&self, p: Point3) -> (f64, f64) {
        let theta = (-p.y).acos();
        let phi = (-p.z).atan2(p.x) + consts::PI;
        let u = phi / consts::TAU;
        let v = theta / consts::PI;

        (u, v)
    }
}

impl object::Hittable for Sphere {
    fn hit(
        &self,
        r: &Ray,
        ray_t: &interval::Interval,
        _rng: &mut rngs::SmallRng,
    ) -> Option<object::HitRecord> {
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
        let mut root = (-half_b - discriminant.sqrt()) / a;
        if !ray_t.contains(root) {
            root = (-half_b + discriminant.sqrt()) / a;
            if !ray_t.contains(root) {
                return None;
            }
        }

        let intersection = r.at(root);
        let outward_normal = (intersection - self.centre) / self.radius;
        let (u, v) = self.get_uv(outward_normal);
        let mut hitrec =
            object::HitRecord::new(intersection, outward_normal, root, u, v, &self.mat);
        hitrec.set_face_normal(r, outward_normal);
        Some(hitrec)
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}

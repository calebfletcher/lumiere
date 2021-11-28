use std::rc::Rc;

use crate::{aabb::AABB, ray::Ray, vec3::Vec3, Point3};

use super::Hittable;

#[derive(Debug)]
pub struct RotateY {
    object: Rc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: AABB,
}

impl RotateY {
    /// Creates a new rotation transformation with the specified angle in degrees
    pub fn new(object: Rc<dyn Hittable>, angle: f64) -> Self {
        let rads = angle.to_radians();
        let sin_theta = rads.sin();
        let cos_theta = rads.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1 - i) as f64 * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1 - j) as f64 * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1 - k) as f64 * bbox.z.min;

                    let new_x = cos_theta * x + sin_theta * z;
                    let new_z = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(new_x, y, new_z);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: AABB::from_points(min, max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
    ) -> Option<super::HitRecord> {
        // Change the ray from world space to object space
        let mut origin = r.origin;
        let mut direction = r.direction;

        origin.x = self.cos_theta * r.origin.x - self.sin_theta * r.origin.z;
        origin.z = self.sin_theta * r.origin.x + self.cos_theta * r.origin.z;

        direction.z = self.sin_theta * r.direction.x + self.cos_theta * r.direction.z;
        direction.x = self.cos_theta * r.direction.x - self.sin_theta * r.direction.z;

        let rotated = Ray::new(origin, direction, r.time);

        // Determine if an intersection occurs in object space
        let mut hitrec = self.object.hit(&rotated, ray_t)?;

        // Change the intersection point from object space to world space
        let mut hit_point = hitrec.point;
        hit_point.x = self.cos_theta * hitrec.point.x + self.sin_theta * hitrec.point.z;
        hit_point.z = -self.sin_theta * hitrec.point.x + self.cos_theta * hitrec.point.z;

        let mut normal = hitrec.normal;
        normal.x = self.cos_theta * hitrec.normal.x + self.sin_theta * hitrec.normal.z;
        normal.z = -self.sin_theta * hitrec.normal.x + self.cos_theta * hitrec.normal.z;

        hitrec.point = hit_point;
        hitrec.normal = normal;

        Some(hitrec)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

use crate::{aabb::AABB, interval, material, ray::Ray, vec3::Vec3, Point3};

use super::object;

#[derive(Debug)]
pub struct MovingSphere {
    name: String,
    centre_0: Point3,
    #[allow(dead_code)]
    centre_1: Point3,
    centre_vec: Vec3,
    radius: f64,
    mat: Box<dyn material::Material>,
    aabb: AABB,
}

impl MovingSphere {
    pub fn new(
        name: String,
        centre_0: Point3,
        centre_1: Point3,
        radius: f64,
        mat: Box<dyn material::Material>,
    ) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box0 = AABB::from_points(centre_0 - rvec, centre_0 + rvec);
        let box1 = AABB::from_points(centre_1 - rvec, centre_1 + rvec);

        Self {
            name,
            centre_0,
            centre_1,
            centre_vec: centre_1 - centre_0,
            radius,
            mat,
            aabb: AABB::from_boxes(&box0, &box1),
        }
    }

    fn centre(&self, time: f64) -> Vec3 {
        self.centre_0 + self.centre_vec * time
    }
}

impl object::Hittable for MovingSphere {
    fn hit(&self, r: &Ray, ray_t: &interval::Interval) -> Option<object::HitRecord> {
        let oc = r.origin - self.centre(r.time);
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
        let outward_normal = (intersection - self.centre(r.time)) / self.radius;
        let mut hitrec = object::HitRecord::new(intersection, outward_normal, root, &self.mat);
        hitrec.set_face_normal(r, outward_normal);
        Some(hitrec)
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}

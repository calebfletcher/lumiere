use std::{fmt, sync::Arc};

use rand::rngs;

use crate::{aabb::AABB, interval, material, ray::Ray, vec3::Vec3, Point3};

#[derive(Debug)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
    pub mat: &'a Arc<dyn material::Material>,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        point: Point3,
        normal: Vec3,
        t: f64,
        u: f64,
        v: f64,
        mat: &'a Arc<dyn material::Material>,
    ) -> Self {
        HitRecord {
            point,
            normal,
            t,
            u,
            v,
            front_face: false,
            mat,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.direction.dot(outward_normal) < 0.;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        }
    }
}

pub trait Hittable: fmt::Debug + Send + Sync {
    fn hit(
        &self,
        r: &Ray,
        ray_t: &interval::Interval,
        rng: &mut rngs::SmallRng,
    ) -> Option<HitRecord>;
    fn bounding_box(&self) -> &AABB;
}

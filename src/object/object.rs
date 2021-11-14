use std::fmt;

use crate::{material, ray::Ray, vec3::Vec3, Point3};

#[derive(Debug)]
pub struct HitRecord<'a> {
    pub point: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub mat: &'a Box<dyn material::Material>,
}

impl<'a> HitRecord<'a> {
    pub fn new(point: Point3, normal: Vec3, t: f64, mat: &'a Box<dyn material::Material>) -> Self {
        HitRecord {
            point,
            normal,
            t,
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

pub trait Hittable: fmt::Debug {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord>;
    fn name(&self) -> String;
}

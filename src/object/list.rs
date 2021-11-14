use crate::{interval, ray};

use super::Hittable;

#[derive(Debug)]
pub struct HittableList(Vec<Box<dyn Hittable>>);

impl HittableList {
    pub fn new() -> Self {
        HittableList(Vec::new())
    }

    pub fn clear(&mut self) {
        self.0.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.0.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, ray_t: &interval::Interval) -> Option<super::HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hitrec = None;
        for object in &self.0 {
            let new_interval = interval::Interval::new(ray_t.min, closest_so_far);
            if let Some(temp_hitrec) = object.hit(&r, &new_interval) {
                closest_so_far = temp_hitrec.t;
                hitrec = Some(temp_hitrec);
            }
        }

        hitrec
    }

    fn name(&self) -> String {
        String::from("world")
    }
}

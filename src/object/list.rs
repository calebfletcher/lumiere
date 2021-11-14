use crate::ray;

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
    fn hit(&self, r: &ray::Ray, t_min: f64, t_max: f64) -> Option<super::HitRecord> {
        let mut closest_so_far = t_max;
        let mut hitrec = None;
        for object in &self.0 {
            if let Some(temp_hitrec) = object.hit(&r, t_min, t_max) {
                if temp_hitrec.t < closest_so_far {
                    closest_so_far = temp_hitrec.t;
                    hitrec = Some(temp_hitrec);
                }
            }
        }

        hitrec
    }

    fn name(&self) -> String {
        String::from("world")
    }
}

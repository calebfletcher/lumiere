use crate::{aabb::AABB, interval, ray};

use super::Hittable;

#[derive(Debug)]
pub struct HittableList {
    pub(crate) objects: Vec<Box<dyn Hittable>>,
    pub(crate) bbox: AABB,
}

impl HittableList {
    pub fn new() -> Self {
        HittableList {
            objects: Vec::new(),
            bbox: AABB::new(interval::EMPTY, interval::EMPTY, interval::EMPTY),
        }
    }

    pub fn clear(&mut self) {
        self.objects.clear();
    }

    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.bbox = AABB::from_boxes(&self.bbox, object.bounding_box());
        self.objects.push(object);
    }
}

impl Default for HittableList {
    fn default() -> Self {
        Self::new()
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &ray::Ray, ray_t: &interval::Interval) -> Option<super::HitRecord> {
        let mut closest_so_far = ray_t.max;
        let mut hitrec = None;
        for object in &self.objects {
            let new_interval = interval::Interval::new(ray_t.min, closest_so_far);
            if let Some(temp_hitrec) = object.hit(r, &new_interval) {
                closest_so_far = temp_hitrec.t;
                hitrec = Some(temp_hitrec);
            }
        }

        hitrec
    }

    fn name(&self) -> String {
        String::from("world")
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

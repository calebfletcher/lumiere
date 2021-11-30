use std::rc::Rc;

use rand::rngs;

use crate::{aabb::AABB, interval::Interval, ray::Ray, vec3::Vec3};

use super::Hittable;

#[derive(Debug)]
pub struct Translate {
    object: Rc<dyn Hittable>,
    offset: Vec3,
    bbox: AABB,
}

impl Translate {
    pub fn new(object: Rc<dyn Hittable>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl<'a> Hittable for Translate {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &Interval,
        rng: &mut rngs::SmallRng,
    ) -> Option<super::HitRecord> {
        let offset_r = Ray::new(r.origin - self.offset, r.direction, r.time);

        let mut hitrec = self.object.hit(&offset_r, ray_t, rng)?;

        hitrec.point += self.offset;

        Some(hitrec)
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

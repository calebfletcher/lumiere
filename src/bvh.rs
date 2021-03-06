use std::{cmp, sync::Arc};

use rand::{rngs, Rng};

use crate::{
    aabb::AABB,
    interval::{self, Interval},
    object::{Hittable, HittableList},
};

#[derive(Debug)]
pub struct BVHNode {
    left: Option<Arc<dyn Hittable>>,
    right: Option<Arc<dyn Hittable>>,
    bbox: AABB,
}

impl BVHNode {
    pub fn new(list: HittableList, rng: &mut rngs::SmallRng) -> Self {
        Self::from_objects(list.objects, rng)
    }

    fn from_objects(mut objects: Vec<Arc<dyn Hittable>>, rng: &mut rngs::SmallRng) -> Self {
        let axis: usize = rng.gen_range(0..=2);

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            2 => Self::box_z_compare,
            _ => {
                panic!("unknown axis {}", axis)
            }
        };

        let (left, right) = match objects.len() {
            1 => (Some(objects.pop().unwrap()), None),
            2 => {
                let el1 = objects.pop().unwrap();
                let el2 = objects.pop().unwrap();
                if comparator(&el1, &el2) == cmp::Ordering::Greater {
                    (Some(el1), Some(el2))
                } else {
                    (Some(el2), Some(el1))
                }
            }
            _ => {
                objects.sort_unstable_by(comparator);
                let midpoint = objects.len() / 2;
                let other_elements = objects.split_off(midpoint);

                let left: Arc<dyn Hittable> = Arc::new(BVHNode::from_objects(objects, rng));
                let right: Arc<dyn Hittable> = Arc::new(BVHNode::from_objects(other_elements, rng));

                (Some(left), Some(right))
            }
        };

        let bbox = match (&left, &right) {
            (Some(a), Some(b)) => AABB::from_boxes(a.bounding_box(), b.bounding_box()),
            (Some(a), None) => a.bounding_box().clone(),
            (None, Some(a)) => a.bounding_box().clone(),
            (None, None) => AABB::new(interval::EMPTY, interval::EMPTY, interval::EMPTY),
        };
        Self { left, right, bbox }
    }

    fn box_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>, axis: usize) -> cmp::Ordering {
        a.bounding_box()
            .axis(axis)
            .min
            .partial_cmp(&b.bounding_box().axis(axis).min)
            .expect("NANs encountered in bvh box compare")
    }

    fn box_x_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> cmp::Ordering {
        Self::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> cmp::Ordering {
        Self::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &Arc<dyn Hittable>, b: &Arc<dyn Hittable>) -> cmp::Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BVHNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
        rng: &mut rngs::SmallRng,
    ) -> Option<crate::object::HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        // Check left for hits
        let hit_left = self.left.as_ref().and_then(|b| b.hit(r, ray_t, rng));

        // Check right for hits that are closer than the left's potential hits
        let new_max = hit_left.as_ref().map_or(ray_t.max, |hr| hr.t);
        let new_interval = Interval::new(ray_t.min, new_max);
        let hit_right = self
            .right
            .as_ref()
            .and_then(|b| b.hit(r, &new_interval, rng));

        // Prioritise hit_right, since it was checked with the reduced interval
        match hit_right {
            Some(_) => hit_right,
            None => hit_left,
        }
    }

    fn bounding_box(&self) -> &AABB {
        &self.bbox
    }
}

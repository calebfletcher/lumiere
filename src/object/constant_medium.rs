use std::sync::Arc;

use rand::{rngs, Rng};

use crate::{interval, material, texture, vec3::Vec3, Colour};

use super::{HitRecord, Hittable};

#[derive(Debug)]
pub struct ConstantMedium {
    phase_function: Arc<dyn material::Material>,
    boundary: Arc<dyn Hittable>,
    neg_inv_density: f64,
}

impl ConstantMedium {
    pub fn new(
        boundary: Arc<dyn Hittable>,
        density: f64,
        texture: Arc<dyn texture::Texture>,
    ) -> Self {
        Self {
            phase_function: Arc::new(material::Isotropic::new(texture)),
            boundary,
            neg_inv_density: -1. / density,
        }
    }

    pub fn from_colour(boundary: Arc<dyn Hittable>, density: f64, colour: Colour) -> Self {
        Self {
            phase_function: Arc::new(material::Isotropic::from_colour(colour)),
            boundary,
            neg_inv_density: -1. / density,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
        rng: &mut rngs::SmallRng,
    ) -> Option<super::HitRecord> {
        let mut hitrec1 = self.boundary.hit(r, &interval::UNIVERSE, rng)?;
        let mut hitrec2 = self.boundary.hit(
            r,
            &interval::Interval::new(hitrec1.t + 0.0001, f64::INFINITY),
            rng,
        )?;

        if hitrec1.t < ray_t.min {
            hitrec1.t = ray_t.min;
        }
        if hitrec2.t > ray_t.max {
            hitrec2.t = ray_t.max;
        }

        if hitrec1.t >= hitrec2.t {
            return None;
        }

        if hitrec1.t < 0. {
            hitrec1.t = 0.;
        }

        // TODO: Check this since this is probably already a unit vector
        let ray_length = r.direction.length();
        let distance_inside_boundary = (hitrec2.t - hitrec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * rng.gen::<f64>().ln();

        if hit_distance > distance_inside_boundary {
            return None;
        }

        let t = hitrec1.t + hit_distance / ray_length;
        Some(HitRecord::new(
            r.at(t),
            Vec3::new(1., 0., 0.),
            t,
            0.,
            0.,
            &self.phase_function,
        ))
    }

    fn bounding_box(&self) -> &crate::aabb::AABB {
        self.boundary.bounding_box()
    }
}

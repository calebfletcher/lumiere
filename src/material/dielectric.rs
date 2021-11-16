use rand::Rng;

use crate::{object::HitRecord, ray::Ray, Colour};

use super::{Behaviour, Material, MaterialScatterResult};

#[derive(Debug, Clone)]
pub struct Dielectric {
    ir: f64, // Index of refraction
}

impl Dielectric {
    pub fn new(ir: f64) -> Self {
        Self { ir }
    }

    fn reflectance(&self, cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1. - ref_idx) / (1. + ref_idx)).powi(2);
        r0 + (1. - r0) * (1. - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> MaterialScatterResult {
        let attenuation = Colour::new(1., 1., 1.);
        let refraction_ratio = if hitrec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r.direction.unit();

        let cos_theta = (-unit_direction).dot(hitrec.normal).min(1.);
        let sin_theta = (1. - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let reflectance = self.reflectance(cos_theta, refraction_ratio);
        let direction = match cannot_refract || reflectance > rng.gen() {
            true => unit_direction.reflect(&hitrec.normal.unit()),
            false => unit_direction.refract(&hitrec.normal.unit(), refraction_ratio),
        };
        let scattered = Ray::new(hitrec.point, direction, r.time);
        MaterialScatterResult::new(Behaviour::Scatter, attenuation, scattered)
    }
}

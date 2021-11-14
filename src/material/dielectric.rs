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
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        _rng: &mut rand::rngs::ThreadRng,
    ) -> MaterialScatterResult {
        let attenuation = Colour::new(1., 1., 1.);
        let refraction_ratio = if hitrec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };

        let unit_direction = r.direction.unit();
        let refracted = unit_direction.refract(hitrec.normal.unit(), refraction_ratio);

        let scattered = Ray::new(hitrec.point, refracted);
        MaterialScatterResult::new(Behaviour::Scatter, attenuation, scattered)
    }
}

use crate::{object::HitRecord, ray::Ray, Colour};

use super::{Behaviour, Material, MaterialScatterResult};

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Colour,
}

impl Metal {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        _rng: &mut rand::rngs::ThreadRng,
    ) -> MaterialScatterResult {
        let reflected = r.direction.unit().reflect(hitrec.normal);
        let scattered = Ray::new(hitrec.point, reflected);
        let behaviour = match scattered.direction.dot(hitrec.normal) {
            d if d > 0. => Behaviour::Scatter,
            d if d <= 0. => Behaviour::Absorb,
            _ => Behaviour::Absorb, // This is likely needed for NaNs
        };

        MaterialScatterResult::new(behaviour, self.albedo, scattered)
    }
}

use crate::{object::HitRecord, ray::Ray, vec3::Vec3, Colour};

use super::{Behaviour, Material, MaterialScatterResult};

#[derive(Debug, Clone)]
pub struct Metal {
    albedo: Colour,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Colour, mut fuzziness: f64) -> Self {
        if fuzziness > 1. {
            fuzziness = 1.;
        }
        Self { albedo, fuzziness }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> MaterialScatterResult {
        let reflected = r.direction.unit().reflect(&hitrec.normal);
        let scattered = Ray::new(
            hitrec.point,
            reflected + Vec3::random_in_unit_sphere(rng) * self.fuzziness,
            r.time,
        );
        let behaviour = match scattered.direction.dot(hitrec.normal) {
            d if d > 0. => Behaviour::Scatter,
            d if d <= 0. => Behaviour::Absorb,
            _ => Behaviour::Absorb, // This is likely needed for NaNs
        };

        MaterialScatterResult::new(behaviour, self.albedo, scattered)
    }
}

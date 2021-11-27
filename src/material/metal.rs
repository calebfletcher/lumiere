use std::rc::Rc;

use crate::{
    object::HitRecord,
    ray::Ray,
    texture::{SolidColour, Texture},
    vec3::Vec3,
    Colour,
};

use super::{Behaviour, Material, MaterialScatterResult};

#[derive(Debug)]
pub struct Metal {
    albedo: Rc<dyn Texture>,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Colour, mut fuzziness: f64) -> Self {
        if fuzziness > 1. {
            fuzziness = 1.;
        }
        Self {
            albedo: Rc::new(SolidColour::new(albedo)),
            fuzziness,
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        rng: &mut rand::rngs::SmallRng,
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

        MaterialScatterResult::new(
            behaviour,
            self.albedo.get_value(hitrec.u, hitrec.v, &hitrec.point),
            scattered,
        )
    }
}

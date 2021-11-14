use crate::{object::HitRecord, ray::Ray, Colour};

pub enum Behaviour {
    Scatter,
    Absorb,
}

pub struct MaterialScatterResult {
    pub behaviour: Behaviour,
    pub attenuation: Colour,
    pub scattered: Ray,
}

impl MaterialScatterResult {
    pub fn new(behaviour: Behaviour, attenuation: Colour, scattered: Ray) -> Self {
        MaterialScatterResult {
            behaviour,
            attenuation,
            scattered,
        }
    }
}

pub trait Material {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> MaterialScatterResult;
}

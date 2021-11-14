use std::fmt;

use crate::{object::HitRecord, ray::Ray, Colour};

#[derive(Debug)]
pub enum Behaviour {
    Scatter,
    Absorb,
}

#[derive(Debug)]
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

pub trait Material: fmt::Debug {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> MaterialScatterResult;
}

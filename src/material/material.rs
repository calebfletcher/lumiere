use std::fmt;

use crate::{object::HitRecord, ray::Ray, Colour, Point3};

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
        rng: &mut rand::rngs::SmallRng,
    ) -> MaterialScatterResult;

    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Colour {
        Colour::new(0., 0., 0.)
    }
}

use std::sync::Arc;

use rand::rngs;

use crate::{
    object::HitRecord,
    ray::Ray,
    texture::{SolidColour, Texture},
    vec3::Vec3,
    Colour, Point3,
};

use super::{Behaviour, Material, MaterialScatterResult};

#[derive(Debug)]
pub struct DiffuseLight {
    emit: Arc<dyn Texture>,
}

impl DiffuseLight {
    pub fn new(emit: Arc<dyn Texture>) -> Self {
        Self { emit }
    }

    pub fn from_colour(emit: Colour) -> Self {
        Self {
            emit: Arc::new(SolidColour::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r: &Ray,
        _hitrec: &HitRecord,
        _rng: &mut rngs::SmallRng,
    ) -> MaterialScatterResult {
        MaterialScatterResult::new(
            Behaviour::Absorb,
            Colour::new(0., 0., 0.),
            Ray::new(Point3::new(0., 0., 0.), Vec3::new(0., 0., 0.), 0.),
        )
    }

    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Colour {
        self.emit.get_value(u, v, p)
    }
}

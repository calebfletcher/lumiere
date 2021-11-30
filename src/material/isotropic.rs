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
pub struct Isotropic {
    albedo: Rc<dyn Texture>,
}

impl Isotropic {
    pub fn new(albedo: Rc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn from_colour(albedo: Colour) -> Self {
        Self {
            albedo: Rc::new(SolidColour::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        rng: &mut rand::rngs::SmallRng,
    ) -> MaterialScatterResult {
        MaterialScatterResult::new(
            Behaviour::Scatter,
            self.albedo.get_value(hitrec.u, hitrec.v, &hitrec.point),
            Ray::new(
                hitrec.point,
                Vec3::random_in_unit_sphere(rng).unit(),
                r.time,
            ),
        )
    }
}
use std::sync::Arc;

use rand::rngs;

use crate::{
    object::HitRecord,
    ray::Ray,
    texture::{SolidColour, Texture},
    vec3::Vec3,
    Colour,
};

use super::{Behaviour, Material, MaterialScatterResult};

#[derive(Debug)]
pub struct Lambertian {
    albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new(albedo: Arc<dyn Texture>) -> Self {
        Self { albedo }
    }

    pub fn from_colour(albedo: Colour) -> Self {
        Self {
            albedo: Arc::new(SolidColour::new(albedo)),
        }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        rng: &mut rngs::SmallRng,
    ) -> MaterialScatterResult {
        let mut scatter_direction = hitrec.normal + Vec3::random_in_unit_sphere(rng).unit();

        if scatter_direction.near_zero() {
            scatter_direction = hitrec.normal;
        }

        MaterialScatterResult::new(
            Behaviour::Scatter,
            self.albedo.get_value(hitrec.u, hitrec.v, &hitrec.point),
            Ray::new(hitrec.point, scatter_direction, r.time),
        )
    }
}

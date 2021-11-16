use crate::{object::HitRecord, ray::Ray, vec3::Vec3, Colour};

use super::{Behaviour, Material, MaterialScatterResult};

#[derive(Debug, Clone)]
pub struct Lambertian {
    albedo: Colour,
}

impl Lambertian {
    pub fn new(albedo: Colour) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r: &Ray,
        hitrec: &HitRecord,
        rng: &mut rand::rngs::ThreadRng,
    ) -> MaterialScatterResult {
        let mut scatter_direction = hitrec.normal + Vec3::random_in_unit_sphere(rng).unit();

        if scatter_direction.near_zero() {
            scatter_direction = hitrec.normal;
        }

        MaterialScatterResult::new(
            Behaviour::Scatter,
            self.albedo,
            Ray::new(hitrec.point, scatter_direction, r.time),
        )
    }
}

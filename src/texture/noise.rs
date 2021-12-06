use noise::{NoiseFn, SuperSimplex};

use crate::{Colour, Point3};

use super::Texture;

#[derive(Debug)]
pub struct NoiseTexture {
    noise: SuperSimplex,
    scale: f64,
}

impl NoiseTexture {
    pub fn new() -> Self {
        Self {
            noise: SuperSimplex::new(),
            scale: 1.,
        }
    }

    pub fn with_scale(scale: f64) -> Self {
        Self {
            noise: SuperSimplex::new(),
            scale,
        }
    }

    fn turb(&self, p: &Point3) -> f64 {
        let depth = 7;
        let mut accum = 0.;
        let mut temp_p = *p;
        let mut weight = 1.;
        let weight_diff: f64 = 0.5;
        let max_value = weight * (1. - weight_diff.powi(depth + 1)) / (1. - weight_diff);

        for _ in 0..depth {
            let p_vals: [f64; 3] = [temp_p.x, temp_p.y, temp_p.z];
            accum += weight * self.noise.get(p_vals);
            weight *= weight_diff;
            temp_p *= 2.;
        }

        accum = accum / max_value + 1.; // Turn accum into [0, 2]
        accum /= 2.; // Turn accum into [0, 1]

        assert!(accum >= 0.);

        accum.abs()
    }
}

impl Texture for NoiseTexture {
    fn get_value(&self, _u: f64, _v: f64, p: &Point3) -> Colour {
        Colour::new(1., 1., 1.) * self.turb(&(*p * self.scale))
    }
}

use std::ops::{Range, RangeInclusive};

mod ops;

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }

    pub fn zeros() -> Self {
        Self::new(0., 0., 0.)
    }

    pub fn length_squared(&self) -> f64 {
        self.x.powi(2) + self.y.powi(2) + self.z.powi(2)
    }

    pub fn length(&self) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn unit(&self) -> Self {
        let len = self.length();
        Self {
            x: self.x / len,
            y: self.y / len,
            z: self.z / len,
        }
    }

    pub fn dot(&self, other: Self) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    /// Creates a new vector where each of the components is independently
    /// uniformly sampled within a range. The supplied range is exclusive in
    /// the upper bound.
    pub fn random_in_range(rng: &mut impl rand::Rng, range: Range<f64>) -> Self {
        Self {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range),
        }
    }

    /// Creates a new vector where each of the components is independently
    /// uniformly sampled within a range. The supplied range is inclusive in
    /// the upper bound.
    pub fn random_in_range_inclusive(rng: &mut impl rand::Rng, range: RangeInclusive<f64>) -> Self {
        Self {
            x: rng.gen_range(range.clone()),
            y: rng.gen_range(range.clone()),
            z: rng.gen_range(range),
        }
    }

    pub fn random_in_unit_sphere(rng: &mut impl rand::Rng) -> Self {
        loop {
            let v = Self::random_in_range(rng, -1.0..1.0);
            if v.length_squared() <= 1.0 {
                break v;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let threshold = 1e-8;
        self.length_squared() < threshold
    }

    pub fn reflect(&self, normal: Self) -> Self {
        *self - normal * self.dot(normal) * 2.
    }

    pub fn refract(&self, normal: Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(normal).min(1.);
        let r_out_perp = (*self + normal * cos_theta) * etai_over_etat;
        let r_out_parallel = normal * -(1. - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }
}

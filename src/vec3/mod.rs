use std::ops::{Range, RangeInclusive};

mod ops;

#[derive(Debug, Clone, Copy, PartialEq)]
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
            let v = Self::random_in_range_inclusive(rng, -1.0..=1.0);
            if v.length_squared() <= 1.0 {
                break v;
            }
        }
    }

    pub fn random_in_unit_disk(rng: &mut impl rand::Rng) -> Self {
        loop {
            let v = Self::new(rng.gen_range(-1.0..=1.0), rng.gen_range(-1.0..=1.0), 0.);
            if v.length_squared() <= 1.0 {
                break v;
            }
        }
    }

    pub fn near_zero(&self) -> bool {
        let threshold = 1e-8;
        self.length_squared() < threshold
    }

    /// Reflects a vector based on a surface normal.
    pub fn reflect(&self, normal: &Self) -> Self {
        *self - *normal * self.dot(*normal) * 2.
    }

    /// Refracts a vector based on a surface normal and a ratio of etas.
    /// Both vectors must be unit vectors.
    pub fn refract(&self, normal: &Self, etai_over_etat: f64) -> Self {
        let cos_theta = (-*self).dot(*normal).min(1.);
        let r_out_perp = (*self + *normal * cos_theta) * etai_over_etat;
        let r_out_parallel = *normal * -(1. - r_out_perp.length_squared()).abs().sqrt();
        r_out_perp + r_out_parallel
    }

    pub fn is_close(&self, other: &Self) -> bool {
        self.x.is_close(other.x) && self.y.is_close(other.y) && self.z.is_close(other.z)
    }
}

trait IsClose {
    fn is_close(&self, other: Self) -> bool;
}

impl IsClose for f64 {
    fn is_close(&self, other: Self) -> bool {
        let rt = 1e-05;
        let at = 1e-08;

        let threshold = rt * self.abs() + at;
        (self - other).abs() < threshold
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn reflect_typical_2d() {
        let v = Vec3::new(1., -1., 0.);
        let n = Vec3::new(0., 1., 0.);

        let res = v.reflect(&n);
        let expected = Vec3::new(1., 1., 0.);

        assert!(res.is_close(&expected));
    }

    #[test]
    fn reflect_perpendicular() {
        let v = Vec3::new(0., -1., 0.);
        let n = Vec3::new(0., 1., 0.);

        let res = v.reflect(&n);
        let expected = Vec3::new(0., 1., 0.);

        assert!(res.is_close(&expected));
    }

    #[test]
    fn reflect_parallel() {
        let v = Vec3::new(1., 0., 0.);
        let n = Vec3::new(0., 1., 0.);

        let res = v.reflect(&n);
        let expected = Vec3::new(1., 0., 0.);

        assert!(res.is_close(&expected));
    }

    #[test]
    fn refract_through_air() {
        let v = Vec3::new(2., -1., 1.).unit();
        let n = Vec3::new(0., 1., 0.).unit();

        let res = v.refract(&n, 1.);
        let expected = v;

        assert!(res.is_close(&expected));
    }

    #[test]
    fn refract_through_glass() {
        let v = Vec3::new(2., -1., 1.).unit();
        let n = Vec3::new(0., 1., 0.).unit();

        let res = v.refract(&n, 1.5).unit();

        dbg!(v, res);

        // Slows down in x and z, so y component gets bigger
        assert!(v.x * res.x > 0.); // Sign of z should be preserved
        assert!(v.x.abs() > res.x.abs());
        assert!(v.y * res.y > 0.); // Sign of y should be preserves
        assert!(v.y.abs() < res.y.abs());
        assert!(v.z * res.z > 0.); // Sign of z should be preserved
        assert!(v.z.abs() > res.z.abs());
    }

    #[test]
    fn refract_perpendicular() {
        let v = Vec3::new(0., -1., 0.).unit();
        let n = Vec3::new(0., 1., 0.).unit();

        let res = v.refract(&n, 1.5).unit();

        // Should be unchanged
        assert_eq!(v, res);
    }
}

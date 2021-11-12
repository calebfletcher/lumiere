use std::ops::{
    Add, AddAssign, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign,
};

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
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, idx: usize) -> &Self::Output {
        match idx {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("vec3 index out of bounds"),
        }
    }
}

impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, idx: usize) -> &mut Self::Output {
        match idx {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("vec3 index out of bounds"),
        }
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
            z: self.z * other.z,
        }
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self {
            x: self.x / other.x,
            y: self.y / other.y,
            z: self.z / other.z,
        }
    }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self {
        Self {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
        self.z += other.z;
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        self.x -= other.x;
        self.y -= other.y;
        self.z -= other.z;
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        self.x *= other.x;
        self.y *= other.y;
        self.z *= other.z;
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        self.x /= other.x;
        self.y /= other.y;
        self.z /= other.z;
    }
}

impl Add<f64> for Vec3 {
    type Output = Self;

    fn add(self, other: f64) -> Self {
        Self {
            x: self.x + other,
            y: self.y + other,
            z: self.z + other,
        }
    }
}

impl Sub<f64> for Vec3 {
    type Output = Self;

    fn sub(self, other: f64) -> Self {
        Self {
            x: self.x - other,
            y: self.y - other,
            z: self.z - other,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
            z: self.z * other,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        Self {
            x: self.x / other,
            y: self.y / other,
            z: self.z / other,
        }
    }
}

impl AddAssign<f64> for Vec3 {
    fn add_assign(&mut self, other: f64) {
        self.x += other;
        self.y += other;
        self.z += other;
    }
}

impl SubAssign<f64> for Vec3 {
    fn sub_assign(&mut self, other: f64) {
        self.x -= other;
        self.y -= other;
        self.z -= other;
    }
}

impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, other: f64) {
        self.x *= other;
        self.y *= other;
        self.z *= other;
    }
}

impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, other: f64) {
        self.x /= other;
        self.y /= other;
        self.z /= other;
    }
}

#[cfg(test)]
mod tests {
    use super::Vec3;

    #[test]
    fn new_constructor() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.x, 1.);
        assert_eq!(v.y, 2.);
        assert_eq!(v.z, 3.);
    }

    #[test]
    fn zeros_constructor() {
        let v = Vec3::zeros();
        assert_eq!(v.x, 0.);
        assert_eq!(v.y, 0.);
        assert_eq!(v.z, 0.);
    }

    #[test]
    fn index() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v[0], 1.);
        assert_eq!(v[1], 2.);
        assert_eq!(v[2], 3.);
    }

    #[test]
    fn length_squared() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.length_squared(), 14.);
    }

    #[test]
    fn length() {
        let v = Vec3::new(1., 2., 3.);
        assert_eq!(v.length(), 14.0_f64.sqrt());
    }

    #[test]
    fn neg() {
        let v = -Vec3::new(1., 2., 3.);
        assert_eq!(v.x, -1.);
        assert_eq!(v.y, -2.);
        assert_eq!(v.z, -3.);
    }

    #[test]
    fn add() {
        let v1 = Vec3::new(6., 5., 4.);
        let v2 = Vec3::new(1., 3., 5.);
        let v = v1 + v2;
        assert_eq!(v.x, 7.);
        assert_eq!(v.y, 8.);
        assert_eq!(v.z, 9.);
    }

    #[test]
    fn sub() {
        let v1 = Vec3::new(6., 5., 4.);
        let v2 = Vec3::new(1., 2., 3.);
        let v = v1 - v2;
        assert_eq!(v.x, 5.);
        assert_eq!(v.y, 3.);
        assert_eq!(v.z, 1.);
    }

    #[test]
    fn mul() {
        let v1 = Vec3::new(6., 5., 4.);
        let v2 = Vec3::new(1., 2., 3.);
        let v = v1 * v2;
        assert_eq!(v.x, 6.);
        assert_eq!(v.y, 10.);
        assert_eq!(v.z, 12.);
    }

    #[test]
    fn div() {
        let v1 = Vec3::new(6., 5., 4.);
        let v2 = Vec3::new(1., 2., 3.);
        let v = v1 / v2;
        assert_eq!(v.x, 6.);
        assert_eq!(v.y, 5. / 2.);
        assert_eq!(v.z, 4. / 3.);
    }

    #[test]
    fn mul_f64() {
        let v1 = Vec3::new(6., 5., 4.);
        let v = v1 * 2.0;
        assert_eq!(v.x, 12.);
        assert_eq!(v.y, 10.);
        assert_eq!(v.z, 8.);
    }
}

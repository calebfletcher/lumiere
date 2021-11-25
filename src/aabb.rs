use std::mem;

use crate::{interval::Interval, ray::Ray, Point3};

#[derive(Debug, Clone)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl AABB {
    pub fn new(x: Interval, y: Interval, z: Interval) -> Self {
        Self { x, y, z }
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        Self {
            x: Interval::new(a.x.min(b.x), a.x.max(b.x)),
            y: Interval::new(a.y.min(b.y), a.y.max(b.y)),
            z: Interval::new(a.z.min(b.z), a.z.max(b.z)),
        }
    }

    pub fn from_boxes(a: &Self, b: &Self) -> Self {
        Self {
            x: Interval::from_intervals(&a.x, &b.x),
            y: Interval::from_intervals(&a.y, &b.y),
            z: Interval::from_intervals(&a.z, &b.z),
        }
    }

    pub fn axis(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => {
                panic!("invalid interval index passed into aabb {}", n);
            }
        }
    }

    pub fn hit(&self, r: &Ray, ray_t: &Interval) -> bool {
        for i in 0..3 {
            let inv_d = 1. / r.direction[i];
            let mut t0 = (self.axis(i).min - r.origin[i]) * inv_d;
            let mut t1 = (self.axis(i).max - r.origin[i]) * inv_d;
            if inv_d < 0. {
                mem::swap(&mut t0, &mut t1);
            }

            let ray_tmin = t0.max(ray_t.min);
            let ray_tmax = t1.min(ray_t.max);

            if ray_tmax <= ray_tmin {
                return false;
            }
        }
        true
    }

    pub fn pad(&self) -> Self {
        let delta: f64 = 0.0001;
        let new_x = if self.x.size() >= delta {
            self.x
        } else {
            self.x.expand(delta)
        };
        let new_y = if self.y.size() >= delta {
            self.y
        } else {
            self.y.expand(delta)
        };
        let new_z = if self.z.size() >= delta {
            self.z
        } else {
            self.z.expand(delta)
        };
        Self {
            x: new_x,
            y: new_y,
            z: new_z,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{interval, ray::Ray, vec3::Vec3, Point3};

    use super::AABB;

    #[test]
    fn hit_bb_linear() {
        let p1 = Point3::new(3., 0., 0.);
        let p2 = Point3::new(4., 1., 0.);

        let bb = AABB::from_points(p1, p2);

        let ray_origin = Point3::new(0., 0.5, 0.);
        let ray_direction = Vec3::new(1., 0., 0.);
        let r = Ray::new(ray_origin, ray_direction, 0.);

        assert!(bb.hit(&r, &interval::UNIVERSE));
    }

    #[test]
    fn miss_bb_linear() {
        let p1 = Point3::new(3., 0., 0.);
        let p2 = Point3::new(4., 1., 1.);

        let bb = AABB::from_points(p1, p2);

        dbg!(&bb);

        let ray_origin = Point3::new(0., -0.5, 0.);
        let ray_direction = Vec3::new(1., 0., 0.);
        let r = Ray::new(ray_origin, ray_direction, 0.);

        assert!(!bb.hit(&r, &interval::UNIVERSE));
    }
}

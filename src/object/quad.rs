use std::rc::Rc;

use rand::rngs;

use crate::{aabb::AABB, material, object, vec3::Vec3, Point3};

use super::{Hittable, HittableList};

#[derive(Debug)]
pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    mat: Rc<dyn material::Material>,
    aabb: AABB,
    normal: Vec3,
    d: f64,
    w: Vec3,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Rc<dyn material::Material>) -> Self {
        let aabb = AABB::from_points(q, q + u + v).pad();
        let n = u.cross(v);
        let normal = n.unit();
        let d = normal.dot(q);
        let w = n / n.length_squared();
        Self {
            q,
            u,
            v,
            mat,
            aabb,
            normal,
            d,
            w,
        }
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: &crate::interval::Interval,
        _rng: &mut rngs::SmallRng,
    ) -> Option<super::HitRecord> {
        let denom = self.normal.dot(r.direction.unit());

        // If denominator is effectively zero, then the ray is parallel to the
        // plane the quad is in, and so won't hit it
        if denom.abs() < 1e-8 {
            return None;
        }

        // If the hit point would be outside the valid range, there is not hit
        let t = (self.d - self.normal.dot(r.origin)) / denom;
        if !ray_t.contains(t) {
            return None;
        }

        let intersection = r.at(t);

        let planar_hitpoint_vec = intersection - self.q;
        let alpha = self.w.dot(planar_hitpoint_vec.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpoint_vec));

        // If ray intersection point is not inside the quad, there's no hit
        if alpha < 0. || alpha > 1. || beta < 0. || beta > 1. {
            return None;
        }

        let mut hitrec =
            object::HitRecord::new(intersection, self.normal, t, alpha, beta, &self.mat);
        hitrec.set_face_normal(r, self.normal);
        Some(hitrec)
    }

    fn bounding_box(&self) -> &AABB {
        &self.aabb
    }
}

pub fn new_box<'a>(a: &Point3, b: &Point3, mat: Rc<dyn material::Material>) -> HittableList {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point3::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0., 0.);
    let dy = Vec3::new(0., max.y - min.y, 0.);
    let dz = Vec3::new(0., 0., max.z - min.z);

    // Front
    sides.add(Box::new(Quad::new(
        Point3::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    )));
    // Right
    sides.add(Box::new(Quad::new(
        Point3::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    )));
    // Back
    sides.add(Box::new(Quad::new(
        Point3::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    )));
    // Left
    sides.add(Box::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    )));
    // Top
    sides.add(Box::new(Quad::new(
        Point3::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    )));
    // Bottom
    sides.add(Box::new(Quad::new(
        Point3::new(min.x, min.y, min.z),
        dx,
        dz,
        mat.clone(),
    )));

    sides
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;

    use rand::{rngs, SeedableRng};

    use crate::{interval, material, object::Hittable, ray::Ray, vec3::Vec3, Colour, Point3};

    use super::Quad;

    #[test]
    fn hit_quad_centre() {
        let mat = Rc::new(material::Lambertian::from_colour(Colour::new(1., 0.2, 0.2)));
        let q = Vec3::new(-0.5, -0.5, 1.);
        let u = Vec3::new(1., 0., 0.);
        let v = Vec3::new(0., 1., 0.);
        let quad = Quad::new(q, u, v, mat);

        let ray_origin = Point3::new(0., 0., 0.);
        let ray_direction = Vec3::new(0., 0., 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);

        let hit_result = quad.hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy());

        assert!(hit_result.is_some());
        let hit_result = hit_result.unwrap();

        let expected = Point3::new(0., 0., 1.);
        assert_eq!(hit_result.point, expected);
    }

    #[test]
    fn hit_quad_off_centre() {
        let mat = Rc::new(material::Lambertian::from_colour(Colour::new(1., 0.2, 0.2)));
        let q = Vec3::new(-0.5, -0.5, 1.);
        let u = Vec3::new(1., 0., 0.);
        let v = Vec3::new(0., 1., 0.);
        let quad = Quad::new(q, u, v, mat);

        let ray_origin = Point3::new(0.4, 0.4, 0.);
        let ray_direction = Vec3::new(0., 0., 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);

        let hit_result = quad.hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy());

        assert!(hit_result.is_some());
        let hit_result = hit_result.unwrap();

        let expected = Point3::new(0.4, 0.4, 1.);
        assert_eq!(hit_result.point, expected);
    }

    #[test]
    fn hit_quad_multiple() {
        let mat = Rc::new(material::Lambertian::from_colour(Colour::new(1., 0.2, 0.2)));
        let q = Vec3::new(-0.5, -0.5, 1.);
        let u = Vec3::new(1., 0., 0.);
        let v = Vec3::new(0., 1., 0.);
        let quad = Quad::new(q, u, v, mat);

        let ray_origin = Point3::new(0., 0., 0.);

        // Hit rays

        // Ray 1
        let ray_direction = Vec3::new(0.2, 0., 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_some());
        // Ray 2
        let ray_direction = Vec3::new(-0.2, 0., 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_some());
        // Ray 3
        let ray_direction = Vec3::new(0., 0.2, 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_some());
        // Ray 4
        let ray_direction = Vec3::new(0., -0.2, 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_some());
        // Ray 5
        let ray_direction = Vec3::new(0.2, 0.2, 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_some());
        // Ray 6
        let ray_direction = Vec3::new(0.2, -0.2, 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_some());
        // Ray 7
        let ray_direction = Vec3::new(-0.2, 0.2, 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_some());
        // Ray 8
        let ray_direction = Vec3::new(-0.2, -0.2, 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_some());

        // Miss rays

        // Ray 1
        let ray_direction = Vec3::new(1., 1., 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_none());
        // Ray 2
        let ray_direction = Vec3::new(1., -1., 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_none());
        // Ray 3
        let ray_direction = Vec3::new(-1., 1., 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_none());
        // Ray 4
        let ray_direction = Vec3::new(-1., 1., 1.);
        let r = Ray::new(ray_origin, ray_direction, 0.);
        assert!(quad
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_none());
    }

    #[test]
    fn hit_top() {
        let back_green = Rc::new(material::Lambertian::from_colour(Colour::new(0.2, 1., 0.2)));
        let green = Box::new(Quad::new(
            Point3::new(-2., -2., 0.),
            Vec3::new(4., 0., 0.),
            Vec3::new(0., 4., 0.),
            back_green,
        ));

        let r = Ray {
            origin: Vec3 {
                x: 0.0,
                y: 0.0,
                z: 9.0,
            },
            direction: Vec3 {
                x: 0.0,
                y: 0.5573489911065506,
                z: -0.8302783280999875,
            },
            time: 0.22690750755679256,
        };

        assert!(green
            .hit(&r, &interval::UNIVERSE, &mut rngs::SmallRng::from_entropy())
            .is_none());
    }
}

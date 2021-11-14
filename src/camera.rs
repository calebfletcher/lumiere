use crate::{ray, vec3::Vec3, Point3};

pub struct CameraBuilder {
    origin: Point3,
    aspect_ratio: f64,
    focal_length: f64,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            origin: Point3::new(0., 0., 0.),
            aspect_ratio: 16. / 9.,
            focal_length: 1.,
        }
    }

    pub fn origin(&mut self, origin: Point3) -> &mut Self {
        self.origin = origin;
        self
    }

    pub fn aspect_ratio(&mut self, aspect_ratio: f64) -> &mut Self {
        self.aspect_ratio = aspect_ratio;
        self
    }

    pub fn focal_length(&mut self, focal_length: f64) -> &mut Self {
        self.focal_length = focal_length;
        self
    }

    pub fn build(&mut self) -> Camera {
        let viewport_height = 2.0;
        let viewport_width = self.aspect_ratio * viewport_height;
        let horizontal = Vec3::new(viewport_width, 0., 0.);
        let vertical = Vec3::new(0., viewport_height, 0.);
        let upper_left_corner =
            self.origin - horizontal / 2. + vertical / 2. - Vec3::new(0., 0., self.focal_length);

        Camera {
            origin: self.origin,
            upper_left_corner: upper_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }
}

pub struct Camera {
    origin: Point3,
    upper_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn get_ray(&self, u: f64, v: f64) -> ray::Ray {
        ray::Ray::new(
            self.origin,
            self.upper_left_corner + self.horizontal * u - self.vertical * v - self.origin,
        )
    }
}

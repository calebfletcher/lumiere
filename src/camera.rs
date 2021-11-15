use crate::{ray, vec3::Vec3, Point3};

pub struct CameraBuilder {
    origin: Point3,
    aspect_ratio: f64,
    focal_length: f64,
    fov: f64,
    look_dir: Vec3,
    v_up: Vec3,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            origin: Point3::new(0., 0., -1.),
            aspect_ratio: 16. / 9.,
            focal_length: 1.,
            fov: 40.,
            look_dir: Vec3::new(0., 0., 1.).unit(),
            v_up: Vec3::new(0., 1., 0.),
        }
    }

    pub fn origin(&mut self, origin: Point3) -> &mut Self {
        self.origin = origin;
        self
    }

    pub fn look_dir(&mut self, look_dir: Vec3) -> &mut Self {
        self.look_dir = look_dir;
        self
    }

    pub fn v_up(&mut self, v_up: Vec3) -> &mut Self {
        self.v_up = v_up;
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

    /// Sets the vertical field of view of the camera in degrees.
    pub fn fov(&mut self, fov: f64) -> &mut Self {
        self.fov = fov;
        self
    }

    pub fn build(&mut self) -> Camera {
        let theta = self.fov.to_radians();
        let h = (theta / 2.).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = self.aspect_ratio * viewport_height;

        let w = -self.look_dir.unit();
        let u = self.v_up.cross(w);
        let v = w.cross(u);

        //dbg!(w, u, v);

        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let upper_left_corner = self.origin - horizontal / 2. + vertical / 2. - w;

        Camera {
            origin: self.origin,
            upper_left_corner,
            horizontal,
            vertical,
        }
    }
}

impl Default for CameraBuilder {
    fn default() -> Self {
        Self::new()
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

    pub fn builder() -> CameraBuilder {
        CameraBuilder::new()
    }
}

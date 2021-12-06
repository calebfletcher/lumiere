use rand::{rngs, Rng};

use crate::{ray::Ray, vec3::Vec3, Point3};

pub struct CameraBuilder {
    origin: Point3,
    aspect_ratio: f64,
    aperture: f64,
    focus_dist: f64,
    fov: f64,
    look_dir: Option<Vec3>,
    look_at: Vec3,
    v_up: Vec3,
}

impl CameraBuilder {
    pub fn new() -> Self {
        CameraBuilder {
            origin: Point3::new(0., 0., -1.),
            aspect_ratio: 16. / 9.,
            aperture: 0.,
            focus_dist: 10.,
            fov: 40.,
            look_dir: None,
            look_at: Vec3::new(0., 0., 0.).unit(),
            v_up: Vec3::new(0., 1., 0.),
        }
    }

    pub fn origin(&mut self, origin: Point3) -> &mut Self {
        self.origin = origin;
        self
    }

    pub fn look_dir(&mut self, look_dir: Vec3) -> &mut Self {
        self.look_dir = Some(look_dir);
        self
    }

    pub fn look_at(&mut self, look_at: Vec3) -> &mut Self {
        self.look_at = look_at;
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

    pub fn aperture(&mut self, aperture: f64) -> &mut Self {
        self.aperture = aperture;
        self
    }

    pub fn focus_dist(&mut self, focus_dist: f64) -> &mut Self {
        self.focus_dist = focus_dist;
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

        let look_dir = match self.look_dir {
            Some(look_dir) => look_dir,
            None => self.look_at - self.origin,
        };

        let w = -look_dir.unit();
        let u = self.v_up.cross(w);
        let v = w.cross(u);

        let horizontal = u * viewport_width * self.focus_dist;
        let vertical = v * viewport_height * self.focus_dist;
        let upper_left_corner = self.origin - horizontal / 2. + vertical / 2. - w * self.focus_dist;

        Camera {
            origin: self.origin,
            upper_left_corner,
            horizontal,
            vertical,
            lens_radius: self.aperture / 2.,
            u,
            v,
            w,
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
    lens_radius: f64,
    u: Vec3,
    v: Vec3,
    #[allow(dead_code)]
    w: Vec3,
}

impl Camera {
    /// Gets the ray of the camera with a given normalised pixel coordinates s,t.
    /// s,t is 0,0 at the top left corner, 1,1 in the bottom right corner, 1,0
    /// is the top right corner, and 0,1 is the bottom left corner.
    pub fn get_ray(&self, s: f64, t: f64, rng: &mut rngs::SmallRng) -> Ray {
        let rd = Vec3::random_in_unit_disk(rng) * self.lens_radius;
        let offset = self.u * rd.x + self.v * rd.y;
        Ray::new(
            self.origin + offset,
            (self.upper_left_corner + self.horizontal * s
                - self.vertical * t
                - self.origin
                - offset)
                .unit(),
            rng.gen(),
        )
    }

    pub fn builder() -> CameraBuilder {
        CameraBuilder::new()
    }
}

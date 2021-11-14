use std::io::{self, Write};

use crate::{
    camera::Camera,
    interval,
    material::Behaviour,
    object::{self, Hittable},
    ray::Ray,
    Colour,
};
use rand::{rngs, Rng};

pub struct Scene {
    world: object::HittableList,
    camera: Camera,
    max_depth: usize,
    samples_per_pixel: usize,
    image_width: usize,
    image_height: usize,
}

impl Scene {
    pub fn new(
        world: object::HittableList,
        camera: Camera,
        max_depth: usize,
        samples_per_pixel: usize,
        image_width: usize,
        image_height: usize,
    ) -> Self {
        Self {
            world,
            camera,
            max_depth,
            samples_per_pixel,
            image_width,
            image_height,
        }
    }

    pub fn render(&self, pixel_buffer: &mut [u8], rng: &mut rngs::ThreadRng) -> io::Result<()> {
        #[allow(clippy::needless_range_loop)]
        for row in 0..self.image_height {
            // Print progress indicator
            eprint!("\rScanlines remaining: {} ", self.image_height - row);
            io::stderr().flush()?;

            for col in 0..self.image_width {
                let mut pixel_colour = Colour::zeros();
                for _ in 0..self.samples_per_pixel {
                    let u = (col as f64 + rng.gen::<f64>()) / (self.image_width - 1) as f64;
                    let v = (row as f64 + rng.gen::<f64>()) / (self.image_height - 1) as f64;
                    let r = self.camera.get_ray(u, v);
                    pixel_colour += self.ray_colour(&r, self.max_depth, rng);
                }
                pixel_colour /= self.samples_per_pixel as f64;

                let pixel_offset = row * self.image_width * 3 + col * 3;
                pixel_buffer[pixel_offset + 0] = (pixel_colour.x.sqrt() * 255.999) as u8;
                pixel_buffer[pixel_offset + 1] = (pixel_colour.y.sqrt() * 255.999) as u8;
                pixel_buffer[pixel_offset + 2] = (pixel_colour.z.sqrt() * 255.999) as u8;
            }
        }
        eprintln!("\nRaytracing Completed");

        Ok(())
    }

    fn ray_colour(&self, r: &Ray, depth: usize, rng: &mut rngs::ThreadRng) -> Colour {
        if depth <= 0 {
            return Colour::new(0., 0., 0.);
        }
        match self
            .world
            .hit(r, &interval::Interval::new(0.001, f64::INFINITY))
        {
            Some(hitrec) => {
                // Ray intersects object

                let scatter_result = hitrec.mat.scatter(r, &hitrec, rng);
                match scatter_result.behaviour {
                    Behaviour::Scatter => {
                        scatter_result.attenuation
                            * self.ray_colour(&scatter_result.scattered, depth - 1, rng)
                            * 0.5
                    }
                    Behaviour::Absorb => Colour::new(0., 0., 0.),
                }
            }
            None => {
                // Ray doesn't intersect any objects
                let unit = r.direction.unit();
                let t = 0.5 * (unit.y + 1.0);
                Colour::new(1.0, 1.0, 1.0) * (1.0 - t) + Colour::new(0.5, 0.7, 1.0) * t
            }
        }
    }
}

use indicatif::{ProgressBar, ProgressStyle};
use rand::{seq::SliceRandom, thread_rng, SeedableRng};
use rayon::prelude::*;
use std::io;

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
    background: Colour,
}

impl Scene {
    pub fn new(
        world: object::HittableList,
        camera: Camera,
        max_depth: usize,
        samples_per_pixel: usize,
        image_width: usize,
        image_height: usize,
        background: Colour,
    ) -> Self {
        Self {
            world,
            camera,
            max_depth,
            samples_per_pixel,
            image_width,
            image_height,
            background,
        }
    }

    pub fn render(&self, pixel_buffer: &mut Vec<u8>) -> io::Result<()> {
        let pb = ProgressBar::new(self.image_height as u64);
        pb.set_style(ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {percent}% ({eta_precise})"));

        let mut rows: Vec<usize> = (0..self.image_height).collect();
        let mut cols: Vec<usize> = (0..self.image_width).collect();
        rows.shuffle(&mut thread_rng());
        cols.shuffle(&mut thread_rng());

        let mut pixelbuff: Vec<(usize, Vec<u8>)> = rows
            .par_iter()
            .map(|row| {
                let mut row_buffer = vec![0; self.image_width * 3];

                let mut rng = rngs::SmallRng::from_rng(rand::thread_rng()).unwrap();

                for col in &cols {
                    let pixel_colour = self.render_pixel(*row, *col, &mut rng);

                    let pixel_offset = col * 3;

                    row_buffer[pixel_offset] = pixel_colour.0;
                    row_buffer[pixel_offset + 1] = pixel_colour.1;
                    row_buffer[pixel_offset + 2] = pixel_colour.2;
                }
                (*row, row_buffer)
            })
            .collect();

        pixelbuff.sort_unstable_by_key(|elem| elem.0);

        let pixelbuff: Vec<u8> = pixelbuff.into_iter().flat_map(|el| el.1).collect();

        // TODO: don't use new vec
        pixel_buffer.copy_from_slice(&pixelbuff);

        Ok(())
    }

    fn render_pixel(&self, row: usize, col: usize, rng: &mut rngs::SmallRng) -> (u8, u8, u8) {
        let mut pixel_colour = Colour::zeros();
        for _ in 0..self.samples_per_pixel {
            let u = (col as f64 + rng.gen::<f64>()) / (self.image_width - 1) as f64;
            let v = (row as f64 + rng.gen::<f64>()) / (self.image_height - 1) as f64;
            let r = self.camera.get_ray(u, v, rng);
            pixel_colour += self.ray_colour(&r, self.max_depth, rng);
        }
        pixel_colour /= self.samples_per_pixel as f64;

        let r = (pixel_colour.x.sqrt() * 255.999) as u8;
        let g = (pixel_colour.y.sqrt() * 255.999) as u8;
        let b = (pixel_colour.z.sqrt() * 255.999) as u8;

        (r, g, b)
    }

    fn ray_colour(&self, r: &Ray, depth: usize, rng: &mut rngs::SmallRng) -> Colour {
        if depth == 0 {
            return Colour::new(0., 0., 0.);
        }
        match self
            .world
            .hit(r, &interval::Interval::new(0.001, f64::INFINITY), rng)
        {
            Some(hitrec) => {
                // Ray intersects object
                let emitted = hitrec.mat.emitted(hitrec.u, hitrec.v, &hitrec.point);
                let scatter_result = hitrec.mat.scatter(r, &hitrec, rng);

                match scatter_result.behaviour {
                    Behaviour::Scatter => {
                        emitted
                            + scatter_result.attenuation
                                * self.ray_colour(&scatter_result.scattered, depth - 1, rng)
                    }
                    Behaviour::Absorb => emitted,
                }
            }
            None => {
                // Ray doesn't intersect any objects
                self.background
            }
        }
    }
}

use std::fmt;

pub mod checker;
pub mod image;
pub mod solid;

pub use checker::CheckerTexture;
pub use image::ImageTexture;
pub use solid::SolidColour;

use crate::{Colour, Point3};

pub trait Texture: fmt::Debug {
    fn get_value(&self, u: f64, v: f64, p: &Point3) -> Colour;
}

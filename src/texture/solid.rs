use crate::{Colour, Point3};

use super::Texture;

#[derive(Debug)]
pub struct SolidColour {
    value: Colour,
}

impl SolidColour {
    pub fn new(value: Colour) -> Self {
        Self { value }
    }
}

impl Texture for SolidColour {
    fn get_value(&self, _u: f64, _v: f64, _p: &Point3) -> Colour {
        self.value
    }
}

use std::sync::Arc;

use crate::{Colour, Point3};

use super::{SolidColour, Texture};

#[derive(Debug)]
pub struct CheckerTexture {
    inv_scale: f64,
    even: Arc<dyn Texture>,
    odd: Arc<dyn Texture>,
}

impl CheckerTexture {
    pub fn new(scale: f64, even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> Self {
        Self {
            inv_scale: 1. / scale,
            even,
            odd,
        }
    }

    pub fn from_colours(scale: f64, even: Colour, odd: Colour) -> Self {
        Self {
            inv_scale: 1. / scale,
            even: Arc::new(SolidColour::new(even)),
            odd: Arc::new(SolidColour::new(odd)),
        }
    }
}

impl Texture for CheckerTexture {
    fn get_value(&self, u: f64, v: f64, p: &Point3) -> Colour {
        let x = (self.inv_scale * p.x).floor() as i64;
        let y = (self.inv_scale * p.y).floor() as i64;
        let z = (self.inv_scale * p.z).floor() as i64;

        let is_even = (x + y + z) % 2 == 0;

        if is_even {
            self.even.get_value(u, v, p)
        } else {
            self.odd.get_value(u, v, p)
        }
    }
}

pub mod lambertian;
pub mod material;
pub mod metal;

pub use lambertian::Lambertian;
pub use material::{Behaviour, Material, MaterialScatterResult};
pub use metal::Metal;

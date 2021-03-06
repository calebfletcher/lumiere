#![allow(incomplete_features)]
#![feature(generic_const_exprs)]
#![allow(clippy::borrowed_box, clippy::module_inception)]

pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod image;
pub mod interval;
pub mod material;
pub mod object;
pub mod ray;
pub mod scene;
pub mod texture;
pub mod vec3;

pub type Point3 = vec3::Vec3;
pub type Colour = vec3::Vec3;

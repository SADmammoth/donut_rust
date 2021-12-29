mod affine_transform;
mod simple_transform;

pub use affine_transform::*;
pub use simple_transform::*;

pub struct Angle {
    pub value: i16,
}

pub struct Scale {
  pub x: f64,
  pub y: f64,
}
use super::*;
use crate::{figures::Figure, Point};

pub struct Transform {
  figure: Figure,
  rotation: Angle,
  offset: Point, 
  scale: Scale,
}

impl Transform {
  pub fn new(figure: Figure) -> Transform {
    Transform {
      figure,
      rotation: Angle { value: 0 },
      offset: Point::new(0.0, 0.0, 0),
      scale: Scale {x: 1.0, y: 1.0},
    }
  }

  pub fn rotate(mut self: Self, angle: Angle) -> Transform {
    self.rotation = Angle { value: self.rotation.value + angle.value };

    self
  }

  pub fn offset(mut self: Self, offset: Point) -> Transform {
    self.offset = Point {
      x: self.offset.x + offset.x,
      y: self.offset.y + offset.y,
      intensity: 0
    };

    self
  }

  pub fn scale(mut self: Self, scale: Scale) -> Transform {
    self.scale = Scale {
      x: self.scale.x * scale.x,
      y: self.scale.y * scale.y
    };

    self
  }

  pub fn apply(self: Self) -> Figure {
    affine_transform(self.figure, self.rotation, self.offset, self.scale)
  }
}
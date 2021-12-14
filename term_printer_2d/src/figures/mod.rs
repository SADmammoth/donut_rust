mod rect;
mod line;
use crate::Point;
use std::ops::Fn;

pub use rect::*;
pub use line::*;



pub type PointMapper<Target> = dyn Fn(&Target, f32, f32, Point) -> Option<Point>;

pub struct Figure {
    pub _width: f32,
    pub _height: f32,
    pub _origin: Point,
    pub point_mapper: Box<PointMapper<Figure>>,
}

impl Figure {
  pub fn map(self: &Self, point: Point ) -> Option<Point> {
    (*self.point_mapper)(self, self.width(), self.height(), point)
  }

  pub fn width(self: &Self) -> f32 {
    self._width
  }

  pub fn height(self: &Self) -> f32 {
    self._height
  }
  
  pub fn origin(self: &Self) -> Point {
   self._origin
  }
}


pub type GetWidth =  dyn Fn(&Path) -> f32;
pub type GetHeight =  dyn Fn(&Path) -> f32;
pub type GetOrigin =  dyn Fn(&Path, f32, f32) -> Point;

pub struct Path {
    pub get_width: Box<GetWidth>,
    pub get_height: Box<GetHeight>,
    pub get_origin: Box<GetOrigin>,
    pub points: Vec<Point>,
    pub point_mapper: Box<PointMapper<Path>>,
}

impl Path {
  pub fn map(self: &Self, point: Point ) -> Option<Point> {
    (*self.point_mapper)(self, self.width(), self.height(), point)
  }

  pub fn width(self: &Self) -> f32 {
    (*self.get_width)(self)
  }

  pub fn height(self: &Self) -> f32 {
    (*self.get_height)(self)
  }
  
  pub fn origin(self: &Self) -> Point {
    crate::append_debug_string(format!("{:?}", (*self.get_origin)(self, self.width(), self.height())));
    (*self.get_origin)(self, self.width(), self.height())
  }
}
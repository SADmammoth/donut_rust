mod rect;
mod line;
use crate::{Point, Pixel};
use std::ops::Fn;

pub use rect::*;
pub use line::*;

pub trait Printable {
  fn map(self: &Self, point: Point ) -> Option<Pixel>;
  fn width(self: &Self) -> f32;
  fn height(self: &Self) -> f32;
  fn origin(self: &Self) -> Point;
}

pub type PointMapper<Target> = dyn Fn(&Target, f32, f32, Point) -> Option<Pixel>;

pub struct Figure {
    pub _width: f32,
    pub _height: f32,
    pub _origin: Point,
    pub point_mapper: Box<PointMapper<Figure>>,
}

impl Printable for Figure {
  fn map(self: &Self, point: Point ) -> Option<Pixel> {
    (*self.point_mapper)(self, self.width(), self.height(), point)
  }

  fn width(self: &Self) -> f32 {
    self._width
  }

  fn height(self: &Self) -> f32 {
    self._height
  }
  
  fn origin(self: &Self) -> Point {
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

impl Printable for Path {
  fn map(self: &Self, point: Point) -> Option<Pixel> {
    (*self.point_mapper)(self, self.width(), self.height(), point)
  }

  fn width(self: &Self) -> f32 {
    (*self.get_width)(self)
  }

  fn height(self: &Self) -> f32 {
    (*self.get_height)(self)
  }
  
  fn origin(self: &Self) -> Point {
    (*self.get_origin)(self, self.width(), self.height())
  }
}
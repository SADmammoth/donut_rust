mod rect;
mod line;
mod multipoint_line;
use crate::{Point, Pixel};
use std::ops::Fn;

pub use rect::*;
pub use line::*;
pub use multipoint_line::*;

pub trait Printable {
  fn map(self: &Self, x_index: usize, y_index: usize, point: Point ) -> Option<Pixel>;
  fn width(self: &Self) -> f64;
  fn height(self: &Self) -> f64;
  fn origin(self: &Self) -> Point;
}

pub type PointMapper<Target> = dyn Fn(&Target, f64, f64, usize, usize, Point) -> Option<Pixel>;

pub struct Figure {
    pub _width: f64,
    pub _height: f64,
    pub _origin: Point,
    pub point_mapper: Box<PointMapper<Figure>>,
}

impl Printable for Figure {
  fn map(self: &Self, x_index: usize, y_index: usize, point: Point ) -> Option<Pixel> {
    (*self.point_mapper)(self, self.width(), self.height(), x_index, y_index, point)
  }

  fn width(self: &Self) -> f64 {
    self._width
  }

  fn height(self: &Self) -> f64 {
    self._height
  }
  
  fn origin(self: &Self) -> Point {
   self._origin
  }
}


pub type GetWidth =  dyn Fn(&Path) -> f64;
pub type GetHeight =  dyn Fn(&Path) -> f64;
pub type GetOrigin =  dyn Fn(&Path, f64, f64) -> Point;

pub struct Path {
    pub get_width: Box<GetWidth>,
    pub get_height: Box<GetHeight>,
    pub get_origin: Box<GetOrigin>,
    pub points: Vec<Point>,
    pub point_mapper: Box<PointMapper<Path>>,
}

impl Printable for Path {
  fn map(self: &Self, x_index: usize, y_index: usize, point: Point) -> Option<Pixel> {
    (*self.point_mapper)(self, self.width(), self.height(), x_index, y_index, point)
  }

  fn width(self: &Self) -> f64 {
    (*self.get_width)(self)
  }

  fn height(self: &Self) -> f64 {
    (*self.get_height)(self)
  }
  
  fn origin(self: &Self) -> Point {
    (*self.get_origin)(self, self.width(), self.height())
  }
}
mod rect;
mod line;
use crate::Point;
use std::ops::Fn;

pub use rect::*;
pub use line::*;

pub type PointMapper = dyn Fn(Point) -> Option<Point>;

pub struct Figure {
    pub width: f32,
    pub height: f32,
    pub origin: Point,
    pub point_mapper: Box<PointMapper>,
}

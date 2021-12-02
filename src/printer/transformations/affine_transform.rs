use super::super::figures::Figure;
use super::super::Point;
use super::*;
use core::f32::consts::PI;

pub fn affine_transform(fig: Figure, angle: Angle) -> Figure {
  let mapper = fig.point_mapper;
  let origin = fig.origin.clone();
  Figure {
    point_mapper: Box::new(move |point| -> Point { 
        let mapped_point = (*mapper)(point);
        let radians = angle.value as f32 / 180.0 * PI;
        let new_x = mapped_point.x * radians.cos() - mapped_point.y * radians.sin() + origin.x;
        let new_y = mapped_point.x * radians.sin() + mapped_point.y * radians.cos();
        Point::new(
          if new_x < 0.0 {0.0} else if  new_x > 1.0 {1.0} else {new_x},
          if new_y < 0.0 {0.0} else if  new_y > 1.0 {1.0} else {new_y},
          mapped_point.intensity,
        )
      }),
    ..fig
  }
}

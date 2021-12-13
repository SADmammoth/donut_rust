use super::*;
use crate::{figures::Figure, Point};
use core::f32::consts::PI;

pub fn affine_transform(fig: Figure, angle: Angle, offset: Point, scale: Point) -> Figure {
    let mapper = fig.point_mapper;
    let origin = fig.origin.clone();
    Figure {
        point_mapper: Box::new(move |point| -> Option<Point> {
            let mapped_point = (*mapper)(point).unwrap();
            let radians = angle.value as f32 / 180.0 * PI;
            let new_x = 
                  scale.x * mapped_point.x * radians.cos()
                - mapped_point.y * radians.sin()
                - origin.x * (radians.cos() - 1.0)
                + origin.y * radians.sin()
                + offset.x;
            let new_y = 
                  mapped_point.x * radians.sin() 
                + scale.y * mapped_point.y * radians.cos()
                - origin.x * radians.sin()
                - origin.y * (radians.cos() - 1.0)
                + offset.y;

            if new_x < 0.0 || new_x > 1.0 || new_y < 0.0 || new_y > 1.0 {
                return None;
            }

            Some(Point::new(new_x, new_y, mapped_point.intensity))
        }),
        ..fig
    }
}

use super::*;
use crate::{
    figures::{Figure, Path, Printable},
    Point,
    Pixel,
};
use core::f32::consts::PI;

pub fn affine_transform(fig: Figure, angle: Angle, offset: Point, scale: Scale) -> Figure {
    let mapper = fig.point_mapper;
    Figure {
        point_mapper: Box::new(move |figure, w, h, point| -> Option<Pixel> {
            let origin = figure.origin();
            let mapped_point = (*mapper)(figure, w, h, point).unwrap();
            let radians = angle.value as f32 / 180.0 * PI;
            let new_x = scale.x * mapped_point.get_x() * radians.cos()
                - mapped_point.get_y() * radians.sin()
                - origin.get_x() * (radians.cos() - 1.0)
                + origin.get_y() * radians.sin()
                + offset.get_x();
            let new_y = mapped_point.get_x() * radians.sin() 
                + scale.y * mapped_point.get_y() * radians.cos()
                - origin.get_x() * radians.sin()
                - origin.get_y() * (radians.cos() - 1.0)
                + offset.get_y();

            if new_x < 0.0 || new_x > 1.0 || new_y < 0.0 || new_y > 1.0 {
                return None;
            }

            Some(Pixel::new(new_x, new_y, mapped_point.get_intensity()))
        }),
        ..fig
    }
}

pub fn affine_transform_path(fig: Path, angle: Angle, offset: Point, scale: Scale) -> Path {
    let origin = fig.origin();
    let new_points = fig
        .points
        .iter()
        .map(|mapped_point| {
            let radians = angle.value as f32 / 180.0 * PI;
            let new_x = scale.x * mapped_point.get_x() * radians.cos()
                - mapped_point.get_y() * radians.sin()
                - origin.get_x() * (radians.cos() - 1.0)
                + origin.get_y() * radians.sin()
                + offset.get_x();
            let new_y = mapped_point.get_x() * radians.sin() 
                + scale.y * mapped_point.get_y() * radians.cos()
                - origin.get_x() * radians.sin()
                - origin.get_y() * (radians.cos() - 1.0)
                + offset.get_y();

            Point::new(
                new_x,
                new_y,
            )
        })
        .collect();
    Path {
        points: new_points,
        ..fig
    }
}

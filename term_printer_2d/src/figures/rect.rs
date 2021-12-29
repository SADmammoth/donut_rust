use super::Figure;
use crate::{Point, Pixel, Intensity};

pub fn rect(left_top: Point, right_bottom: Point, intensity: Intensity) -> Figure {
    if right_bottom.is_higher_from(&left_top) || left_top.is_to_right_from(&right_bottom) {
        panic!("Please, pass correct corner points for `rect`")
    }

    let width = right_bottom.get_x() - left_top.get_x();
    let height = left_top.get_y() - right_bottom.get_y();

    let origin = Point::new(left_top.get_x() + (width / 2.0), left_top.get_y() - (height / 2.0));
    let x_mapper = move |x: f64| (width * x) + left_top.get_x();

    let y_mapper = move |y: f64| (height * y) + right_bottom.get_y();

    Figure {
        _width: width,
        _height: height,
        _origin: origin,
        point_mapper: Box::new(move |_,_,_, point| {
            Some(Pixel::new(
                x_mapper(point.get_x()),
                y_mapper(point.get_y()),
                intensity,
            ))
        }),
    }
}

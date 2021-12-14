use super::Figure;
use crate::Point;

pub fn rect(left_top: Point, right_bottom: Point, intensity: u8) -> Figure {
    if right_bottom.is_higher_from(&left_top) || left_top.is_to_right_from(&right_bottom) {
        panic!("Please, pass correct corner points for `rect`")
    }

    let width = right_bottom.x - left_top.x;
    let height = left_top.y - right_bottom.y;

    let origin = Point::new(left_top.x + (width / 2.0), left_top.y - (height / 2.0), 0);
    let x_mapper = move |x: f32| (width * x) + left_top.x;

    let y_mapper = move |y: f32| (height * y) + right_bottom.y;

    Figure {
        _width: width,
        _height: height,
        _origin: origin,
        point_mapper: Box::new(move |_,_,_, point| {
            Some(Point {
                x: x_mapper(point.x),
                y: y_mapper(point.y),
                intensity,
            })
        }),
    }
}

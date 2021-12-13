use super::Figure;
use crate::Point;

pub fn line(start: Point, end: Point, line_width: f32) -> Figure {
    if line_width <= 0.0 {
        panic!("Please, pass correct `line_width` param");
    }

    let width = (end.x - start.x).abs();
    let height = (end.y - start.y).abs();
    let origin = Point::new(  if start.x < end.x {
            (width / 2.0) + start.x
        } else {
            (width / 2.0) + end.x
        }, if start.y < end.y {
            (height / 2.0) + start.y
        } else {
            (height / 2.0) + end.y
        }, 0); //FIXME
    let x_mapper = move |x: f32| {
        if start.x < end.x {
            (width * x) + start.x
        } else {
            start.x - (width * x)
        }
    };
    let y_mapper = move |y: f32| {
        if start.y < end.y {
            (height * y) + start.y
        } else {
            start.y - (height * y)
        }
    };
                println!("{:?}", (width, height));

    Figure {
        width,
        height,
        origin,
        point_mapper: Box::new(move |point| {
            if (point.x - point.y).abs() >= line_width {
                None
            } else {
                Some(Point {
                    x: x_mapper(point.x),
                    y: y_mapper(point.y),
                    intensity: start.intensity,
                })
            }
        }),
    }
}

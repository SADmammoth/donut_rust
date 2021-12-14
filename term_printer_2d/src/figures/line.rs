use super::Path;
use super::PointMapper;
use crate::Point;

pub fn line(start: Point, end: Point, line_width: f32) -> Path {
    if line_width <= 0.0 {
        panic!("Please, pass correct `line_width` param");
    }

    let get_width = Box::new(move |figure: &Path| (figure.points[0].x - figure.points[0].x).abs());
    let get_height = Box::new(move |figure: &Path| (figure.points[1].y - figure.points[0].y).abs());

    let get_origin = Box::new(move |figure: &Path, width: f32, height: f32| {
        Point::new(
            if start.x < end.x {
                (width / 2.0) + start.x
            } else {
                (width / 2.0) + end.x
            },
            if start.y < end.y {
                (height / 2.0) + start.y
            } else {
                (height / 2.0) + end.y
            },
            0,
        )
    }); //FIXME

    let x_mapper = move |x: f32, start: Point, end: Point, width: f32| {
        if start.x < end.x {
            (width * x) + start.x
        } else {
            start.x - (width * x)
        }
    };

    let y_mapper = move |y: f32, start: Point, end: Point, height: f32| {
        if start.y < end.y {
            (height * y) + start.y
        } else {
            start.y - (height * y)
        }
    };

    Path {
        get_width,
        get_height,
        get_origin,
        points: vec![start, end],
        point_mapper: Box::new(move |figure, width, height, point| {
            let line_dispersion = if width > height {
                line_width * 0.015 * (width / height / 0.8)
            } else {
                line_width * 0.015 * (height / width / 0.8)
            };
            if (point.x - point.y).abs() >= line_dispersion {
                None
            } else {
                Some(Point {
                    x: x_mapper(point.x, figure.points[0], figure.points[1], width),
                    y: y_mapper(point.y, figure.points[0], figure.points[1], height),
                    intensity: start.intensity,
                })
            }
        }),
    }
}

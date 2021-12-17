use super::Path;
use crate::{Intensity, Pixel, Point};

pub fn line(start: Point, end: Point, intensity: Intensity, line_width: (f32, f32)) -> Path {
    if line_width.0 <= 0.0 || line_width.1 <= 0.0  {
        panic!("Please, pass correct `line_width` param");
    }

    let get_width =
        Box::new(move |figure: &Path| (figure.points[1].get_x() - figure.points[0].get_x()).abs());
    let get_height =
        Box::new(move |figure: &Path| (figure.points[1].get_y() - figure.points[0].get_y()).abs());

    let get_origin = Box::new(move |_: &Path, width: f32, height: f32| {
        Point::new(
            if start.get_x() < end.get_x() {
                (width / 2.0) + start.get_x()
            } else {
                (width / 2.0) + end.get_x()
            },
            if start.get_y() < end.get_y() {
                (height / 2.0) + start.get_y()
            } else {
                (height / 2.0) + end.get_y()
            },
        )
    });

    let x_mapper = move |x: f32, start: Point, end: Point, width: f32| {
        if start.get_x() < end.get_x() {
            (width * x) + start.get_x()
        } else {
            start.get_x() - (width * x)
        }
    };

    let y_mapper = move |y: f32, start: Point, end: Point, height: f32| {
        if start.get_y() < end.get_y() {
            (height * y) + start.get_y()
        } else {
            start.get_y() - (height * y)
        }
    };

    Path {
        get_width,
        get_height,
        get_origin,
        points: vec![start, end],
        point_mapper: Box::new(move |figure, width, height, point| {
            let line_dispersion = if width > height {
                width / height 
            } else {
                height / width
            };

            if (point.get_x() - point.get_y()).abs() >= (line_width.0 - line_width.1).abs() * line_dispersion {
                None
            } else {
                Some(Pixel::new(
                    x_mapper(point.get_x(), figure.points[0], figure.points[1], width),
                    y_mapper(point.get_y(), figure.points[0], figure.points[1], height),
                    intensity,
                ))
            }
        }),
    }
}

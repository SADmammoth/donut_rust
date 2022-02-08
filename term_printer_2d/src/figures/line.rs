use super::Path;
use crate::{Intensity, Pixel, Point};

pub fn line(start: Point, end: Point, intensity: Intensity, line_width: (f64, f64)) -> Path {
    if line_width.0 <= 0.0 || line_width.1 <= 0.0 {
        panic!("Please, pass correct `line_width` param");
    }

    let get_width =
        Box::new(move |figure: &Path| (figure.points[1].get_x() - figure.points[0].get_x()).abs());
    let get_height =
        Box::new(move |figure: &Path| (figure.points[1].get_y() - figure.points[0].get_y()).abs());

    let get_origin = Box::new(move |_: &Path, width: f64, height: f64| {
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

    let x_mapper = move |x: f64, start: Point, end: Point, width: f64| {
        if start.get_x() < end.get_x() {
            (width * x) + start.get_x()
        } else {
            start.get_x() - (width * x)
        }
    };

    let y_mapper = move |y: f64, start: Point, end: Point, height: f64| {
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
        point_mapper: Box::new(move |figure, width, height, x_index, y_index, point| {
            let new_x = x_mapper(point.get_x(), figure.points[0], figure.points[1], width);
            let new_y = y_mapper(point.get_y(), figure.points[0], figure.points[1], height);

            if new_x > 1.0 || new_x < 0.0 || new_y > 1.0 || new_y < 0.0 {
                None
            } else if is_point(point, x_index, y_index, width, height, line_width) {
                Some(Pixel::new(new_x, new_y, intensity))
            } else {
                None
            }
        }),
    }
}

pub fn calculate_bounds_resolution(width: f64, height: f64, line_width: (f64, f64)) -> (f64, f64) {
      ((width / line_width.0).ceil(),  (height / line_width.1).ceil())
}

pub fn calculate_line_dispersion(width: f64, height: f64, line_width: (f64, f64)) -> (f64, f64) {
    let (width_res, height_res) = calculate_bounds_resolution(width, height, line_width);

    if width_res < height_res {
        (line_width.0, (height_res / width_res) * line_width.1)
    } else if width_res > height_res {
        ((width_res / height_res) * line_width.0, line_width.1)
    } else {
        (line_width.0, line_width.1)
    }
}

pub fn is_point(
    point: Point,
    x_index: usize,
    y_index: usize,
    width: f64,
    height: f64,
    line_width: (f64, f64),
) -> bool {
    let line_dispersion = calculate_line_dispersion(width, height, line_width);

    let (width_res, height_res) = calculate_bounds_resolution(width, height, line_width);

    if y_index == 0 && x_index == 0 
       || y_index == height_res as usize - 1 && x_index == width_res as usize - 1 {
       true
    } else if width_res > height_res {
      point.get_x() > ((y_index as f64) * line_dispersion.0 / width)
      && point.get_x() <= ((y_index as f64 + 1.0) * line_dispersion.0 / width)
    } else {
      point.get_y() > ((x_index as f64) * line_dispersion.1 / height)
      && point.get_y() <= ((x_index as f64 + 1.0) * line_dispersion.1 / height)
    }
}

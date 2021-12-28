use super::Path;
use crate::{Intensity, Pixel, Point};

pub fn calculate_line_dispersion(width: f32, height: f32, line_width: (f32, f32)) -> (f32, f32){
  let width_res = (width / line_width.0).round();
  let height_res = (height / line_width.1).round();

  // crate::debugger::append_debug_string(format!("{:?}",line_width));

  if width_res < height_res {
    (line_width.0, (height_res / width_res) * line_width.1)
  } else if width_res > height_res {
    ((width_res / height_res) * line_width.0, line_width.1)
  } else {
    (line_width.0, line_width.1)
  }
}

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
            (width * x) + end.get_x()
        }
    };

    let y_mapper = move |y: f32, start: Point, end: Point, height: f32| {
        if start.get_y() < end.get_y() {
            (height * y) + start.get_y()
        } else {
            (height * y) + end.get_y()
        }
    };

    Path {
        get_width,
        get_height,
        get_origin,
        points: vec![start, end],
        point_mapper: Box::new(move |figure, width, height, point| {
            let line_dispersion = calculate_line_dispersion(width, height, line_width);
            let x = (point.get_x() * width / line_dispersion.0);
           
            let new_x = x_mapper(point.get_x(), figure.points[0], figure.points[1], width);
            let new_y = y_mapper(point.get_y(), figure.points[0], figure.points[1], height);
            
            if new_x > 1.0 || new_x < 0.0 || new_y > 1.0 || new_y < 0.0 {
              None
            } else if point.get_y() >= ((x  - 1.0 )* line_dispersion.1 / height)  && point.get_y() < (x * line_dispersion.1 / height) {
                 Some(Pixel::new(
                 new_x,new_y,
                    intensity,
                ))
            } else {
              //  Some(Pixel::new(
              //    new_x, new_y,
              //       Intensity::new(1),
              //   ))
              None
            }
        }),
    }
}

use super::Path;
use crate::{Intensity, Point};
use super::{line};

pub fn multipoint_line(points: Vec<Point>, intensity: Intensity, line_width: (f64, f64)) -> Path {
    if line_width.0 <= 0.0 || line_width.1 <= 0.0 {
        panic!("Please, pass correct `line_width` param");
    }

	let min_x_point = points.clone().into_iter().min_by(|a, b| a.get_x().partial_cmp(&b.get_x()).unwrap()).unwrap();
    let max_x_point = points.clone().into_iter().max_by(|a, b| a.get_x().partial_cmp(&b.get_x()).unwrap()).unwrap();
    let min_y_point = points.clone().into_iter().min_by(|a, b| a.get_y().partial_cmp(&b.get_y()).unwrap()).unwrap();
    let max_y_point = points.clone().into_iter().max_by(|a, b| a.get_y().partial_cmp(&b.get_y()).unwrap()).unwrap();

    let get_width =
        Box::new(move |_: &Path| (max_x_point.get_x() - min_x_point.get_x()).abs());
    let get_height =
        Box::new(move |_: &Path| (max_y_point.get_y() - min_y_point.get_y()).abs());

    let get_origin = Box::new(move |_: &Path, width: f64, height: f64| {
        Point::new(
            min_x_point.get_x() + (max_x_point.get_x() - min_x_point.get_x()),
            min_y_point.get_y() + (max_y_point.get_y() - min_y_point.get_y()),
        )
    });

    let mut line_segments:Vec<Path> = vec![];

    for point_i in 0..(points.len() - 1) {
        line_segments.push(
            line(points[point_i], points[point_i+1], intensity, line_width)
        )
    }
    
    Path {
        get_width,
        get_height,
        get_origin,
        points,
        point_mapper: Box::new(move |figure, width, height, x_index, y_index, point| {

            for segment in line_segments.iter() {
                if let Some(point) = (segment.point_mapper)(figure, width, height, x_index, y_index, point){
                    return Some(point);
                }
            }

            None
        }),
    }
}

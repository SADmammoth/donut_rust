use super::Printable;
use super::line;
use super::Path;
use crate::{Point, Intensity};


pub fn path(points: Vec<Point>, intensity: Intensity, line_width: (f64, f64)) -> Path {
  let get_width = Box::new(|figure: &Path| {
	  let (min_x, max_x) = minmax(&figure.points, Box::new(|a, b| {a.get_x() < b.get_x()}));
	  return (max_x.get_x() - min_x.get_x()).abs();
  });

  let get_height = Box::new(|figure: &Path| {
    let (min_y, max_y) = minmax(&figure.points, Box::new(|a, b| {a.get_y() < b.get_y()}));
    return (max_y.get_y() - min_y.get_y()).abs();
    });

  let get_origin = Box::new(|figure: &Path, _: f64, _: f64| {
	  let (min_x, max_x) = minmax(&figure.points, Box::new(|a, b| {a.get_x() < b.get_x()}));
	  let (min_y, max_y) = minmax(&figure.points, Box::new(|a, b| {a.get_y() < b.get_y()}));
	  return Point::new((max_x.get_x() - min_x.get_x()).abs() / 2.0, (max_y.get_y() - min_y.get_y()).abs() / 2.0);
  });

  Path {
	  points,
	  get_width,
	  get_height,
    get_origin,
    point_mapper: Box::new(move |figure, width, height, x_index, y_index, point| {
      let relative_points = points_to_relative_points(&figure.points);

      for line in lines.iter() {
        if let Some(pixel) = line.map(x_index, y_index, point) {
          return Some(pixel);
        }
      }

      return None;
  }),
  }
}

fn minmax<T>(vector: &Vec<T>, lt: Box<dyn Fn(&T, &T) -> bool> ) -> (&T, &T)  {
  vector.iter().fold((&vector[0], &vector[0]), |minmax_res, item| {
    if lt(&minmax_res.0, item) {
      (item, &minmax_res.1)
    } else if lt(item, &minmax_res.1) {
      (&minmax_res.0, item)
    } else {
      minmax_res
    }
  }, )
}

fn get_lines(points: &Vec<Point>, intensity: Intensity, line_width:(f64, f64)) -> Vec<Path> {
  let mut lines: Vec<Path> = vec![];

  let mut start_point = 0;
  let mut end_point = 1;

  while end_point < points.len() {
    lines.push(
      line(points[start_point], points[end_point], intensity, line_width)
    );

    start_point+=1;
    end_point+=1;
  }

  lines
}

fn points_to_relative_points(points: &Vec<Point>) -> Vec<Point> {
  let (min_x, max_x) = minmax(points, Box::new(|a, b| {a.get_x() < b.get_x()}));
  let (min_y, max_y) = minmax(points, Box::new(|a, b| {a.get_y() < b.get_y()}));
  
  points.iter().map(|point|{

    Point::new(
      (point.get_x() - min_x.get_x()) / (max_x.get_x() - min_x.get_x()),
      (point.get_y() - min_y.get_y()) / (max_y.get_y() - min_y.get_y())
    )
  }).collect()
}

fn is_line_point() {
  
}
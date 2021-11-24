use super::figures::Figure;
use super::Point;

#[derive(Debug)]
pub struct CanvasPoint {
  x: usize,
  y: usize,
  intensity: u8,
}

// pub fn get_length_on_canvas(point1: f32, point2: f32, canvas_dimension: usize) -> usize {
//   mul_usize_f32(&canvas_dimension, &point2) - mul_usize_f32(&canvas_dimension, &point1)
// }

pub fn format_matrix(mat: Vec<Vec<char>>) -> String {
 mat.iter().map(| row | {
    format!("{:}\n", &row.iter().collect::<String>())
  }).collect::<String>()
}

pub fn convert_to_canvas_points(points: &Vec<Point>,  canvas_width: usize, canvas_height: usize) -> Vec<CanvasPoint> {
  points.iter().map(| point | {CanvasPoint{x: mul_usize_f32(&canvas_width, &point.x), y: mul_usize_f32(&canvas_height, &point.y), intensity: point.intensity}}).collect()
}

pub fn convert_figure_to_canvas_points(figure: Figure,  canvas_width: usize, canvas_height: usize) -> Vec<CanvasPoint> {
  let steps_x = mul_usize_f32(&canvas_width, &figure.width);
  let step_x = 1f32 / mul_usize_f32(&canvas_width, &figure.width) as f32;
  let steps_y = mul_usize_f32(&canvas_height, &figure.height);
  let step_y = 1f32 / mul_usize_f32(&canvas_height, &figure.height) as f32;
  let mut points = vec![];

    for y in 0..steps_y {
      for x in 0..steps_x {
        points.push((*figure.point_mapper)(Point{x: x as f32 * step_x, y: y as f32 * step_y, intensity: 0 }))
      };

      println!("{:?}", (steps_y, canvas_width, figure.width, step_y));
    };

    println!("{:}", points.len());

  println!("{:?}", (points.last(), canvas_height as f32 * 0.44444445));  
  convert_to_canvas_points(&points, canvas_width, canvas_height)
}

fn mul_usize_f32(a: &usize, b: &f32) -> usize {
  ((*a as f32) * b).round() as usize
}

pub fn get_print_matrix(mut prev_matrix: Vec<Vec<u8>>, canvas_width: usize, canvas_height: usize, canvas_points: &Vec<CanvasPoint>) -> Vec<Vec<u8>> {
  
  canvas_points.iter().for_each(|point| {
    if prev_matrix[point.y][point.x] != 0 {
      prev_matrix[point.y][point.x] = ((prev_matrix[point.y][point.x] + point.intensity) as f32 / 2 as f32).round() as u8;
      return;
    }
    prev_matrix[point.y][point.x] = point.intensity;
  });

  prev_matrix
}
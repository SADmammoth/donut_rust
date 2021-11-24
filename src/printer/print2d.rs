use super::Point;

#[derive(Debug)]
pub struct CanvasPoint {
  x: usize,
  y: usize,
  intensity: u8,
}

pub fn format_matrix(mat: Vec<Vec<char>>) -> String {
 mat.iter().map(| row | {
    format!("{:}\n", &row.iter().collect::<String>())
  }).collect::<String>()
}


pub fn conver_to_canvas_points(points: &Vec<Point>,  canvas_width: usize, canvas_height: usize) -> Vec<CanvasPoint> {
  points.iter().map(| point | {CanvasPoint{x: mul_usize_f32(&canvas_width, &point.x), y: mul_usize_f32(&canvas_height, &point.y), intensity: point.intensity}}).collect()

}

fn mul_usize_f32(a: &usize, b: &f32) -> usize {
  ((*a as f32) * b).round() as usize
}

pub fn get_print_matrix(canvas_width: usize, canvas_height: usize, canvas_points: &Vec<CanvasPoint>) -> Vec<Vec<u8>> {
  let mut mat: Vec<Vec<u8>> = vec![vec![0; canvas_width]; canvas_height];
  
  canvas_points.iter().for_each(|point| {
    if mat[point.y][point.x] != 0 {
      mat[point.y][point.x] = (mat[point.y][point.x] + point.intensity) / 2;
      return;
    }
    mat[point.y][point.x] = point.intensity;
  });

  mat
}
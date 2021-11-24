#[derive(Debug)]
pub struct Point {
  x: f32,
  y: f32,
}

impl Point {
  pub fn new(x:f32, y:f32) -> Point {
    if (x > 1.0 || y > 1.0) || (x < 0.0 || y < 0.0)  {
      panic!("Point coordinates must be between 1 and 0")
    }

    Point{x, y}
  }
}
#[derive(Debug)]
struct CanvasPoint {
  x: usize,
  y: usize,
}


pub fn print2d(points: &Vec<Point>) {
  let (term_w, term_h) = term_size::dimensions_stdout().unwrap();
  println!("{:}", conver_to_path(points, term_w, (term_w as f32 * 0.4).round() as usize));
}

fn conver_to_path(points: &Vec<Point>,  canvas_width: usize, canvas_height: usize) -> String {
  let canvas_points: Vec<CanvasPoint> = points.iter().map(| point | {CanvasPoint{x: mul_usize_f32(&canvas_width, &point.x), y: mul_usize_f32(&canvas_height, &point.y)}}).collect();

  get_print_matrix(canvas_width, canvas_height, &canvas_points).iter().map(| row | {
    format!("{:}\n", &row.iter().collect::<String>())
  }).collect::<String>()
}

fn mul_usize_f32(a: &usize, b: &f32) -> usize {
  ((*a as f32) * b).round() as usize
}

fn get_print_matrix(canvas_width: usize, canvas_height: usize, canvas_points: &Vec<CanvasPoint>) -> Vec<Vec<char>> {
  let mut mat: Vec<Vec<char>> = vec![vec!['.'; canvas_width]; canvas_height];
  
  canvas_points.iter().for_each(|point| {
    mat[point.y][point.x] = '#';
  });

  mat
}
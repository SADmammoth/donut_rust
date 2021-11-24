use super::print2d::*;
use super::point::*;

pub fn print_points(points: &Vec<Point>) {
  let intensity_map = vec![' ', '.', '*', '+', '=', '#'];

  let (term_w, _) = term_size::dimensions_stdout().unwrap();
  let (canvas_w, canvas_y) = (term_w, (term_w as f32 * 0.4).round() as usize);
  let canvas_points = conver_to_canvas_points(points, canvas_w,canvas_y);

  let mat = get_print_matrix(canvas_w, canvas_y, &canvas_points).iter().map(|row| {
    row.iter().map(|i| { intensity_map[*i as usize] }).collect::<Vec<char>>()
  }).collect::<Vec<Vec<char>>>();
  
  println!("{:}", format_matrix(mat));
} 
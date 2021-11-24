use super::print2d::*;
use super::point::*;
use super::figures::*;
 
pub struct Printer {
  mat: Option<Vec<Vec<u8>>>,
}

impl Printer {
  pub fn new() -> Printer {
    Printer { mat: None }
  }

pub fn print_points(self: &mut Self, points: &Vec<Point>) {
  let intensity_map = vec![' ', '.', '*', '+', '=', '#'];

  let (term_w, _) = term_size::dimensions_stdout().unwrap();
  let (canvas_w, canvas_y) = (term_w, (term_w as f32 * 0.4).round() as usize);
  let canvas_points = convert_to_canvas_points(points, canvas_w,canvas_y);

  if let None = self.mat {
    self.mat =  Some(vec![vec![0; canvas_w]; canvas_y]);
  }

  let new_mat = get_print_matrix(self.mat.clone().unwrap(), canvas_w, canvas_y, &canvas_points);

  let print_mat = format_matrix(new_mat.iter().map(|row| {
    row.iter().map(|i| { intensity_map[*i as usize] }).collect::<Vec<char>>()
  }).collect::<Vec<Vec<char>>>());

  self.mat = Some(new_mat);
  
  println!("{:}", print_mat);
} 

pub fn print_figure(self: &mut Self, figure: Figure) {
  let intensity_map = vec![' ', '.', '*', '+', '=', '#'];

  let (term_w, _) = term_size::dimensions_stdout().unwrap();
  let (canvas_w, canvas_y) = (term_w, (term_w as f32 * 0.4).round() as usize);
  let canvas_points = convert_figure_to_canvas_points(figure, canvas_w, canvas_y);

  if let None = self.mat {
    self.mat =  Some(vec![vec![0; canvas_w]; canvas_y]);
  }

  let new_mat = get_print_matrix(self.mat.clone().unwrap(), canvas_w, canvas_y, &canvas_points);

  let print_mat = format_matrix(new_mat.iter().map(|row| {
    row.iter().map(|i| { intensity_map[*i as usize] }).collect::<Vec<char>>()
  }).collect::<Vec<Vec<char>>>());

  self.mat = Some(new_mat);
  
  println!("{:}", print_mat);
} 
}
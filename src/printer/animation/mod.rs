use super::Canvas;
use std::{thread, time};
use thread::*;
use super::Printer;
mod frame_animation;
use frame_animation::FrameAnimation;
use super::super::*;
use super::super::print2d::*;
use super::super::transformations::*;
use super::super::figures::*;
use super::*;

#[derive(Clone)]
pub struct Frame {
  pub matrix: Canvas,
}

pub struct Animator {
  background: Canvas,
  printer: Printer,
  animation: Option<FrameAnimation>,
}

impl Animator {
  pub fn new_with_printer_background(printer: Printer) -> Animator {
    Animator {
      background: printer.get_current_mat(),
      printer,
      animation: None, 
    }
  }

  pub fn new(background: Canvas, printer: Printer) -> Animator {
    Animator {
      background,
      printer,
      animation: None,
    }
  }

  pub fn by_frame(mut self: Self/*, frames: &Frame[]*/){
    let mut printer = self.printer;
    let mut background = self.background.clone();

    let mut angle = 0;

    let frames: Vec<Frame> = vec![];

    while angle <= 360 {
        let rectangle = rect(Point::new(0.33, 0.66, 0), Point::new(0.66, 0.33, 0), 3);
        frames.push(
          Frame {
            matrix: get_figure_matrix(
                &affine_transform(rectangle, Angle{ value: angle }  )
              )
          });
        angle += 5;
    }

    self.animation = Some(FrameAnimation::new(frames));

    self.animation.take().unwrap().start(move |frame| {
      printer.print;
    });
  }
}
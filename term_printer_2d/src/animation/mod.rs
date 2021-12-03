mod frame_animation;
mod timer;

use super::*;
use crate::figures::*;
use crate::print2d::*;
use crate::transformations::*;
use crate::*;
pub use frame_animation::AnimationTime;
use frame_animation::FrameAnimation;
use std::sync::{Arc, Mutex};
use std::{thread, time};
use thread::*;

#[derive(Clone)]
pub struct Frame {
    pub matrix: Canvas,
}

pub struct Animator {
    background: Canvas,
    printer: Printer,
}

impl Animator {
    pub fn new_with_printer_background(printer: Printer) -> Animator {
        Animator {
            background: printer.get_current_mat(),
            printer,
        }
    }

    pub fn new(background: Canvas, printer: Printer) -> Animator {
        Animator {
            background,
            printer,
        }
    }

    pub fn by_frame(
        mut self: Self, /*, frames: &Frame[]*/
        animationTime: AnimationTime,
    ) -> Animator {
        let mut printer = self.printer;
        let mut background = self.background.clone();

        let mut angle = 0;

        let mut frames: Vec<Frame> = vec![];

        while angle <= 360 {
            let rectangle = rect(Point::new(0.33, 0.66, 0), Point::new(0.66, 0.33, 0), 3);
            frames.push(Frame {
                matrix: printer
                    .get_figure_matrix(&affine_transform(rectangle, Angle { value: angle })),
            });
            angle += 5;
        }

        let mut animation: FrameAnimation =
            FrameAnimation::new(frames, Arc::new(Mutex::new(printer)));

        self.printer = animation.start(animationTime);

        self
    }

    pub fn return_printer(self: Self) -> Printer {
        self.printer
    }
}

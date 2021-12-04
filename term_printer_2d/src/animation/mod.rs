mod frame_animation;
mod timer;

use crate::{figures::*, transformations::*, Canvas, Point, Printer};
use frame_animation::FrameAnimation;
use std::sync::{Arc, Mutex};

pub use frame_animation::AnimationTime;

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
        animation_time: AnimationTime,
    ) -> Animator {
        let printer = self.printer;
        let _background = self.background.clone();

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

        let animation: FrameAnimation = FrameAnimation::new(frames, Arc::new(Mutex::new(printer)));

        self.printer = animation.start(animation_time);

        self
    }

    pub fn return_printer(self: Self) -> Printer {
        self.printer
    }
}

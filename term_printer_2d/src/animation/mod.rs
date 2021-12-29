mod frame_animation;
mod timer;

use crate::{Canvas, Printer};
use frame_animation::FrameAnimation;
use std::sync::{Arc, Mutex};

pub use frame_animation::AnimationTime;

pub enum FPS {
  Fixed(u64),
  Default
}

const DEFAULT_FPS_VALUE: u64 = 24;

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
        mut self: Self, frames: Arc<Vec<Frame>>,
        fps: FPS,
        animation_time: AnimationTime,
    ) -> Animator {
        let printer = self.printer;
        let _background = self.background.clone();

        let animation: FrameAnimation = FrameAnimation::new(frames, Arc::new(Mutex::new(printer)));

        self.printer = animation.start(fps, animation_time);

        self
    }

    pub fn return_printer(self: Self) -> Printer {
        self.printer
    }
}

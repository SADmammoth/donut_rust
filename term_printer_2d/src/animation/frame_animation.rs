use super::{timer::Timer, Frame, FPS, DEFAULT_FPS_VALUE};
use crate::Printer;
use std::sync::{Arc, Mutex};
use std::{time};

pub enum AnimationTime {
    Infinite,
    Milliseconds(u64),
}


pub struct FrameAnimation {
    frames: Arc<Vec<Frame>>,
    timer: Arc<Mutex<Timer>>,
    current_frame_index: usize,
    printer: Arc<Mutex<Printer>>,
}

impl FrameAnimation {
    pub fn new(frames: Arc<Vec<Frame>>, printer: Arc<Mutex<Printer>>) -> FrameAnimation {
        FrameAnimation {
            frames,
            timer: Arc::new(Mutex::new(Timer::new())),
            current_frame_index: 0,
            printer,
        }
    }

    fn next_frame(self: &mut Self) -> &Frame {
        if self.current_frame_index >= self.frames.len() - 1 {
            self.current_frame_index = 0;
        } else {
            self.current_frame_index += 1;
        }

        &self.frames[self.current_frame_index]
    }

    pub fn start(mut self: Self, fps: FPS, time: AnimationTime) -> Printer {
        let printer = Arc::clone(&self.printer);
        let returned_printer = Arc::clone(&self.printer);
        let timer_mutex = Arc::clone(&self.timer);

        let mut timer = timer_mutex.lock().unwrap();

        let timer_interval = match fps {
          FPS::Fixed(fps_value) => { self.frames.len() as u64 / fps_value * 1000 }
          FPS::Default => {self.frames.len() as u64 / DEFAULT_FPS_VALUE * 1000 }
        };

        timer.start(time::Duration::from_millis(timer_interval), move || {
            let frame = self.next_frame();
            printer.lock().unwrap().print_matrix(&frame.matrix);
        });

        match time {
            AnimationTime::Infinite => timer.infinite(),
            AnimationTime::Milliseconds(ms) => timer.stop_after(ms),
        };

        Arc::try_unwrap(returned_printer)
            .unwrap()
            .into_inner()
            .unwrap()
    }
}

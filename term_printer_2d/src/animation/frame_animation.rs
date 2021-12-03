use super::super::*;
use super::timer::Timer;
use super::*;
use std::sync::{Arc, Mutex};

pub enum AnimationTime {
    Infinite,
    Milliseconds(u64),
}

pub struct FrameAnimation {
    frames: Vec<Frame>,
    timer: Arc<Mutex<Timer>>,
    current_frame_index: usize,
    printer: Arc<Mutex<Printer>>,
}

impl FrameAnimation {
    pub fn new(frames: Vec<Frame>, printer: Arc<Mutex<Printer>>) -> FrameAnimation {
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

    pub fn start(mut self: Self, time: AnimationTime) -> Printer {
        let printer = Arc::clone(&self.printer);
        let returned_printer = Arc::clone(&self.printer);
        let mut timer_mutex = Arc::clone(&self.timer);

        let mut timer = timer_mutex.lock().unwrap();

        timer.start(move || {
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

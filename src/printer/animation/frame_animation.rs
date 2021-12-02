use std::{sync, thread, time};
use std::sync::atomic::{AtomicBool, Ordering};
use super::super::*;
use super::*;

pub struct FrameAnimation {
    handle: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
    frames: Vec<Frame>,
}

impl FrameAnimation {
    pub fn new(frames: Vec<Frame>) -> FrameAnimation {
        FrameAnimation {
            handle: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
            frames 
        }
    }

    pub fn start<F>(&mut self, print: F)
        where F: 'static + Send + FnMut(Frame) -> ()
    {
        self.alive.store(true, Ordering::SeqCst);

        let alive = self.alive.clone();

        self.handle = Some(thread::spawn(move || {
            let delay = time::Duration::from_millis(100);
            let mut print = print;
            let frame = 0;
            while alive.load(Ordering::SeqCst) {
                print(self.frames[frame].clone());
                frame += 1;
                thread::sleep(delay);
            }
        }));
        
        self.handle
            .take().expect("Called stop on non-running thread")
            .join().expect("Could not join spawned thread");
    }

    pub fn stop(&mut self) {
        self.alive.store(false, Ordering::SeqCst);
    }
}
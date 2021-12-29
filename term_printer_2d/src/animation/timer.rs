use std::sync::atomic::{AtomicBool, Ordering};
use std::{sync, thread, time};

pub struct Timer {
    handle: Option<thread::JoinHandle<()>>,
    alive: sync::Arc<AtomicBool>,
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            handle: None,
            alive: sync::Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn start<F>(self: &mut Self, delay: time::Duration, fun: F)
    where
        F: 'static + Send + FnMut() -> (),
    {
        self.alive.store(true, Ordering::SeqCst);

        let alive = self.alive.clone();

        self.handle = Some(thread::spawn(move || {
            let mut fun = fun;

            while alive.load(Ordering::SeqCst) {
                fun();
                thread::sleep(delay);
            }
        }));
    }

    pub fn infinite(self: &mut Self) {
        self.handle
            .take()
            .expect("Called stop on non-running thread")
            .join()
            .expect("Could not join spawned thread");
    }

    pub fn stop_after(self: &mut Self, time: u64) {
        let delay = time::Duration::from_millis(time);
        thread::sleep(delay);

        self.stop();
    }

    pub fn stop(self: &mut Self) {
        self.alive.store(false, Ordering::SeqCst);
        self.handle
            .take()
            .expect("Called stop on non-running thread")
            .join()
            .expect("Could not join spawned thread");
    }
}

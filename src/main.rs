mod printer;
use printer::figures::*;
use printer::Printer;
use printer::*;
use printer::transformations::*;
use printer::animation::*;
use std::{thread, time};

fn main() {
    let mut printer = Printer::new();
    let mut animator = Animator::new_with_printer_background(printer);

    printer = animator.by_frame(AnimationTime::Milliseconds(10000)).return_printer();
    printer.wipe();
    thread::sleep(time::Duration::from_millis(1000));
    
    let mut animator2 = Animator::new_with_printer_background(printer);

    animator2.by_frame(AnimationTime::Milliseconds(10000));
    
}

use std::{thread, time};
use term_printer_2d::{animation::*, Printer, figures::rect, Point, transformations::* };
use std::sync::{Arc};

fn main() {
    let mut printer = Printer::new();

    let mut angle = 0;

    let mut frames: Vec<Frame> = vec![];

    while angle <= 360 {
        let rectangle = rect(Point::new(0.33, 0.66, 0), Point::new(0.66, 0.33, 0), 3);
        frames.push(Frame {
            matrix: printer.get_figure_matrix(&affine_transform(rectangle, Angle { value: angle })),
        });
        angle += 5;
    }

    let frames = Arc::new(frames);

    let animator = Animator::new_with_printer_background(printer);

    printer = animator
        .by_frame(Arc::clone(&frames), AnimationTime::Milliseconds(10000))
        .return_printer();
    
    printer.wipe();
    thread::sleep(time::Duration::from_millis(1000));

    let animator2 = Animator::new_with_printer_background(printer);

    animator2.by_frame(Arc::clone(&frames), AnimationTime::Milliseconds(10000));
}

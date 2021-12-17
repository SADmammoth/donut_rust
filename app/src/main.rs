// use std::{thread, time};
use std::sync::Arc;
use term_printer_2d::{animation::*, figures::line, transformations::*, Intensity, Point, Printer};

fn main() {
    let mut printer = Printer::new();

    let mut pos = 0.0;
    let mut angle = 20;
    let mut scale = 1.0;

    let mut frames: Vec<Frame> = vec![];

    while pos < 1.0 && angle < 360 {
        let mut rectangle = line(
            Point::new(0.5, 0.2),
            Point::new(0.5, 0.8),
            Intensity::new(4),
            printer.relative(1),
        );
        rectangle = affine_transform_path(
            rectangle,
            Angle { value: angle },
            Point::new(pos, pos),
            Scale { x: scale, y: scale },
        );
        frames.push(Frame {
            matrix: printer.get_figure_matrix(Box::new(rectangle)),
        });
        // pos += 0.01;
        angle += 5;
        // if scale >= 2.0 && scale > 0.0
        // {
        //   scale -= 0.01
        // } else {
        //   scale += 0.01;
        // }
    }

    let frames = Arc::new(frames);

    let animator = Animator::new_with_printer_background(printer);

    printer = animator
        .by_frame(Arc::clone(&frames), AnimationTime::Infinite)
        .return_printer();

    // printer.wipe();
    // thread::sleep(time::Duration::from_millis(1000));

    // let animator2 = Animator::new_with_printer_background(printer);

    // animator2.by_frame(Arc::clone(&frames), AnimationTime::Milliseconds(10000));    //
}

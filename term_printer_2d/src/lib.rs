mod point;
mod pixel;
mod term_print;

pub mod animation;
pub mod figures;
pub mod print2d;
pub mod transformations;

pub use point::*;
pub use pixel::*;
pub use term_print::{Printer};
// use term_print::CHAR_ASPECT_RATIO;

pub type Canvas = Vec<Vec<u8>>;

mod debugger;
#[allow(unused_imports)]
use debugger::{get_debug_string, update_debug_string, append_debug_string};
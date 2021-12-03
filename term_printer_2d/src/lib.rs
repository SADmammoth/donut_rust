pub mod animation;
pub mod figures;
mod point;
pub mod print2d;
mod term_print;
pub mod transformations;

pub use point::*;
pub use term_print::*;

pub type Canvas = Vec<Vec<u8>>;

mod print2d;
mod point;
mod term_print;
pub mod figures;
pub mod transformations;
pub mod animation;

pub use term_print::*;
pub use point::*;

pub type Canvas =  Vec<Vec<u8>>;
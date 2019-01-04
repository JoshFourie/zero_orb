#![feature(type_ascription)]
pub mod transform;
pub mod code_gen;
pub mod field;
pub mod knowledge;
pub mod common;

pub use zksnark::*;
pub use zksnark::field::z251::Z251;
pub use num;

// Notes:
// Knowledge: init the struct with empty vecs appears wasteful as it incurs additinoal heap allocation, potentially find alternate solution.
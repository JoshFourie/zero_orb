#![feature(type_ascription)]

pub mod transform;
pub mod code;
pub mod knowledge;
pub mod common;
pub mod interface;

pub use zksnark::*;
pub use zksnark::groth16::fr::FrLocal;
pub use num;

// Notes:
// Knowledge: init the struct with empty vecs appears wasteful as it incurs additinoal heap allocation, potentially find alternate solution.
// TODO: init tests as temporary folder.
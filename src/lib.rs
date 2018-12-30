pub mod transform;
pub mod code_gen;
pub mod field;
pub mod knowledge;
pub mod common;

use serde::{Serialize, Deserialize};

pub use zksnark::*;
pub use zksnark::field::Field;
pub use num;

pub struct InboundData {
    steps: usize,
    sleep: usize,
    mindulfness: usize,
    calories: usize,
    tag: Vec<u8>,
}



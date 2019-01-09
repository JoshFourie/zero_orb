#![feature(type_ascription)]
pub mod transform;
pub mod code;
pub mod knowledge;
pub mod common;
pub mod interface;

pub use zksnark::{
    *, 
    groth16::{
        fr::{
            FrLocal, 
            G1Local, 
            G2Local
        },
    },
};
pub use crate::{
    knowledge::{
        Knowledge, 
        Marker
    },
    interface::{
        Andromeda,
        InterOperable,
        Transportable
    },
    common::{
        CommonReference, 
        Common,
    },
};

// Notes:
// Knowledge: init the struct with empty vecs appears wasteful as it incurs additinoal heap allocation, potentially find alternate solution.
// TODO: init tests as temporary folder.
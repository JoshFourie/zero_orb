pub mod transform;
pub mod code_gen;

use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use serde_json::{to_string, from_str};

pub use zksnark::*;
pub use zksnark::field::z251::Z251;


pub struct InboundData {
    steps: usize,
    sleep: usize,
    mindulfness: usize,
    calories: usize,
    tag: Vec<u8>,
}

pub struct CommonReference {
    pub code: Vec<u8>,
    pub qap: QAP<CoefficientPoly<Z251>>,
    pub sg1: SigmaG1<Z251>,
    pub sg2: SigmaG2<Z251>,
}

pub trait Commoner {
    type CRS;
    // update to get request when able: fn get(tag: Vec<u8>) -> Result<bool, String>;
    fn new(paths: [&Path; 4]);
    // update paths to tag.
    fn read(paths: [&Path; 4]) -> Self;
}

pub trait Knowledgeable {
    fn new(
        witness_bits: Option<Vec<u8>>, variable_bits: Option<Vec<u8>>,
        witness_num: Option<Vec<u8>>, variable_num: Option<Vec<u8>>,
        tag: Vec<u8>, paths: [&Path; 4]) -> Self;
    fn check(self, verify_num: Option<Vec<u8>>, verify_bits: Option<Vec<u8>>, paths: [&Path; 4]) -> bool;
    fn as_bits(&self) -> Vec<u8>;
}

impl Commoner for CommonReference {
    type CRS = CommonReference;
    //  should be a get request.
    fn new(paths: [&Path; 4]) {
        let qap: QAP<CoefficientPoly<Z251>> = ASTParser::try_parse(
            &read_to_string(paths[0]).unwrap()
        ).unwrap().into();
        let (sg1, sg2) = groth16::setup(&qap);
        File::create(paths[1]).unwrap().write_all(
            to_string(&qap).unwrap().as_bytes()
        ).unwrap();
        File::create(paths[2]).unwrap().write_all(
            to_string(&sg1).unwrap().as_bytes()
        ).unwrap();
        File::create(paths[3]).unwrap().write_all(
            to_string(&sg2).unwrap().as_bytes()
        ).unwrap();
    }
    fn read(paths: [&Path; 4]) -> Self::CRS {
        let code = read_to_string(paths[0]).unwrap().as_bytes().to_vec();
        let qap : QAP<CoefficientPoly<Z251>> = from_str(
            &read_to_string(paths[1]).unwrap()
        ).unwrap();
        let sg1 : SigmaG1<Z251> = from_str(
            &read_to_string(paths[2]).unwrap()
        ).unwrap();
        let sg2 : SigmaG2<Z251> = from_str(
            &read_to_string(paths[3]).unwrap()
        ).unwrap();
        Self {
            code,
            qap,
            sg1,
            sg2
        }
    }
}
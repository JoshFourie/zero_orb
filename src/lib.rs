use std::fs::{File, read_to_string};
use std::io::Write;
use std::path::Path;
use itertools::Itertools;
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

pub trait IntoNumField {
    type Field;
    fn collect_num_field(self) -> Result<Self::Field, ()>;
}

impl IntoNumField for Vec<u8> {
    type Field = Vec<Z251>;
    fn collect_num_field(self) -> Result<Self::Field, ()> {
        Ok(self.into_iter()
            .map(|num: u8| Z251::from(num as usize))
            .collect::<Vec<_>>()
        )
    }
}

impl IntoNumField for Option<Vec<u8>> {
    type Field = Vec<Z251>;
    fn collect_num_field(self) -> Result<Self::Field, ()> {
        match self {
            Some(x) => {
                return Ok(
                    x.into_iter()
                        .map(|num: u8| Z251::from(num as usize))
                        .collect::<Vec<_>>()
                )
            },
            None => Err(()),
        }
    }
}

pub trait IntoBitsField {
    type Field;
    fn collect_bit_field(self) -> Result<Self::Field, ()>;
} 

impl IntoBitsField for Vec<u8> {
    type Field = Vec<Z251>;
    fn collect_bit_field(self) -> Result<Self::Field, ()> {
        let bit_array = self.into_iter().map(|mut num| {
            let mut bits: [u8; 8] = [0; 8];
            for i in 0..8 {
                bits[i] = num % 2;
                num = num >> 1;
            }
            bits
        }).collect::<Vec<_>>();
        Ok(
            bit_array.into_iter()
                .map(|a| {
                a.iter().map(|&n| {
                    assert!(n < 251);
                    Z251 { inner: (n) as u8 }
                }).collect::<Vec<_>>()        
            }).concat()
        )
    }
}

impl IntoBitsField for Option<Vec<u8>> {
    type Field = Vec<Z251>;
    fn collect_bit_field(self) -> Result<Self::Field, ()> {
        match self {
            Some(x) => {
                let bit_array = x.into_iter()
                    .map(|mut num| {
                        let mut bits: [u8; 8] = [0; 8];
                        for i in 0..8 {
                            bits[i] = num % 2;
                            num = num >> 1;
                        }
                        bits
                    }).collect::<Vec<_>>();
                return Ok(
                    bit_array.into_iter()
                        .map(|a| {
                        a.iter().map(|&n| {
                            assert!(n < 251);
                            Z251 { inner: (n) as u8 }
                        }).collect::<Vec<_>>()        
                    }).concat()
                )
            },
            None => Err(())
        }
    }
}

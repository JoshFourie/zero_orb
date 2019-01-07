use zksnark::{
    SigmaG1, SigmaG2, 
    CoefficientPoly, QAP, 
    field::Field,
    groth16::{
        EllipticEncryptable,
        Random,
        circuit::{ASTParser, TryParse}
    },        
};
use std::{
    str::FromStr, 
    fs::File,
    io::Write,
    ops::{Add, Sub},
    iter::Sum,
};
use serde::{Serialize, Deserialize};
use serde_derive::{Serialize, Deserialize};
use serde_json::to_string;
use crate::transform::wrapped_groth::{WrappedQAP, WrappedDummyRep};

// Struct to access the code, QAP, SigmaG1 and SigmaG2 values.
// F => a field, G => G1, H => G2.
#[derive(Serialize, Deserialize)]
pub struct CommonReference<F, G, H> {
    pub code: String,
    pub qap: QAP<CoefficientPoly<F>>,
    pub sg1: SigmaG1<G>,
    pub sg2: SigmaG2<H>,
}

// TODO: impl clone trait for RefFinder.
// a struct for holding the relevant ref_str for the CommonReference struct.
#[derive(Serialize, Deserialize)]
pub struct RefFinder {
    pub code: String,
    pub qap: String,
    pub sg1: String,
    pub sg2: String,
}

pub trait Common {
    fn new(code: String) -> Self;
    fn read(s: &String) -> Self;
}

// defining the generics per the CommonReference struct.
// require <'de> lifetime for deserialization.
// TypeParameters extracted from extern crate zksnark::groth16::mod.rs.
impl<F, G, H> Common for CommonReference<F, G, H> 
where
    for <'de> F: Field
        + From<usize>
        + FromStr
        + EllipticEncryptable<G1 = G, G2 = H>
        + Random
        + Serialize 
        + Deserialize<'de>,
    for <'de> G: Add<Output=G> + Sub<Output=G> + Sum + Copy + Serialize + Deserialize<'de>,
    for <'de> H: Add<Output=H> + Sum + Copy + Serialize + Deserialize<'de>,
{
    // generates and returns a new CommonReference Struct with new QAP, G1 and G2 values.
    fn new(code: String) -> Self {
        let qap: QAP<CoefficientPoly<F>> = WrappedQAP::from(
            WrappedDummyRep(
                ASTParser::try_parse(
                    &code
                ).expect("ASTParser::try_parse into WrappedDummyRep")
            )
        ).0;
        let (sg1, sg2) = zksnark::groth16::setup(&qap);
        Self {
            code: code,
            qap: qap,
            sg1: sg1,
            sg2: sg2
        }
    }

    // returns a CommonReference struct from json string.
    fn read(s: &String) -> Self {
        use serde_json::from_str;
        let crs: CommonReference<F, G, H> = from_str(&s).expect("CommonReference::read() parsing CommonReference struct from String");
        crs    
    }
}

impl<F, G, H> CommonReference<F, G, H> 
where
    F: Field
        + From<usize>
        + FromStr
        + EllipticEncryptable<G1 = G, G2 = H>
        + Random
        + Serialize,
    G: Add<Output=G> + Sub<Output=G> + Sum + Copy + Serialize,
    H: Add<Output=H> + Sum + Copy + Serialize,
{
    // used to derive a CommonReference struct without reading from file or generating new values.
    pub fn init(c: String, q: QAP<CoefficientPoly<F>>, g1: SigmaG1<G>, g2: SigmaG2<H>) -> Self{
        Self {
            code: c,
            qap: q,
            sg1: g1,
            sg2: g2,
        }  
    }

    // improve for error handling/checking file integrity.
    // writes the CommonReference struct to the files given in the Pathfinder.
    pub fn write(&self, ref_str: RefFinder) {
        let _qap_file = File::create(ref_str.qap).expect("CommonReference::write() creating File for QAP from P")
            .write_all(
                to_string(&self.qap).expect("CommonReference::write() parsing QAP as string for writing").as_bytes()
            );
        let _sg1_file = File::create(ref_str.sg1).expect("CommonReference::write() creating File for SigmaG1")
            .write_all(
                to_string(&self.sg1).expect("CommonReference::write() parsing SigmaG1 as string for writing").as_bytes()
            );
        let _sg2_file = File::create(ref_str.sg2).expect("CommonReference::write() creating File for SigmaG2")
            .write_all(
                to_string(&self.sg2).expect("CommonReference::write() parsing SigmaG2 as string for writing").as_bytes()
            );
    }
}

#[test]
fn test_read_reference() {
    use zksnark::{groth16, QAP, field::z251::Z251};
    use serde_json::to_string;
    use std::fs::read_to_string;

    // expected results are that CommonReference::read() will return qap, sg1 and sg2 as before they were written to File.
    let code = read_to_string("src/tests/files/common/simple.zk").expect("internal test err whilst reading code to string");
    let qap: QAP<CoefficientPoly<Z251>> = ASTParser::try_parse(&code).expect("internal test err: ASTParser::try_parse &code unwrapped").into();
    let (sg1, sg2) = groth16::setup(&qap);
    let _crs = CommonReference {
        code: code,
        qap: qap,
        sg1: sg1,
        sg2: sg2,
    };
    let crs: CommonReference<Z251, Z251, Z251> = CommonReference::read(
        &to_string(&_crs).unwrap()
    );
    match crs.code == _crs.code {
        true => {},
        false => panic!("CommonReference: crs.code != code"),
    };
    match to_string(&crs.qap).unwrap() == to_string(&_crs.qap).unwrap() {
        true => {},
        false => panic!("CommonReference: crs.qap != qap"),
    };
    match to_string(&crs.sg1).unwrap() == to_string(&_crs.sg1).unwrap() {
        true => {},
        false => panic!("CommonReference: crs.sg1 != sg1"),
    }; 
    match to_string(&crs.sg2).unwrap() == to_string(&_crs.sg2).unwrap() {
        true => {},
        false => panic!("CommonReference: crs.sg2 != sg2"),
    };
}
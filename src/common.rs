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
    fs::{read_to_string, File},
    io::Write,
    ops::{Add, Sub},
    iter::Sum,
    path::Path,
};
use serde::{Serialize, Deserialize};
use serde_json::to_string;
use crate::transform::wrapped_groth::{WrappedQAP, WrappedDummyRep};

// Struct to access the code, QAP, SigmaG1 and SigmaG2 values.
// F => a field, G => G1, H => G2.
pub struct CommonReference<F, G, H> {
    pub code: Vec<u8>,
    pub qap: QAP<CoefficientPoly<F>>,
    pub sg1: SigmaG1<G>,
    pub sg2: SigmaG2<H>,
}

// TODO: impl clone trait for PathFinder.
// a struct for holding the relevant paths for the CommonReference struct.
pub struct PathFinder<P> {
    pub code: P,
    pub qap: P,
    pub sg1: P,
    pub sg2: P,
}

// defining the generics per the CommonReference struct.
// require <'de> lifetime for deserialization.
// TypeParameters extracted from extern crate zksnark::groth16::mod.rs.
impl<F, G, H> CommonReference<F, G, H> 
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
    pub fn new<P: AsRef<Path>>(paths: PathFinder<P>) -> Self {
        let code = read_to_string(paths.code).expect("read_to_string for code on CommonReference");
        let qap: QAP<CoefficientPoly<F>> = WrappedQAP::from(
            WrappedDummyRep(
                ASTParser::try_parse(
                    &code
                ).expect("ASTParser::try_parse into WrappedDummyRep")
            )
        ).0;
        let (sg1, sg2) = zksnark::groth16::setup(&qap);
        Self {
            code: code.as_bytes().to_vec(),
            qap: qap,
            sg1: sg1,
            sg2: sg2
        }
    }

    // returns a CommonReference struct with values read from file given by the pathfinder. 
    pub fn read<P: AsRef<Path>> (paths: PathFinder<P>) -> Self {
        use serde_json::from_reader;

        let c = read_to_string(paths.code).expect("CommonReference::read() for code");
        let q: QAP<CoefficientPoly<F>> = from_reader(
            File::open(paths.qap).expect("CommonReference::read() opening File for QAP from P")
            ).expect("CommonReference::read() for qap");
        let g1: SigmaG1<G> = from_reader(
            File::open(paths.sg1).expect("CommonReference::read() opening File for G1 from P")
            ).expect("CommonReference::read() for sg1");
        let g2: SigmaG2<H> = from_reader(
            File::open(paths.sg2).expect("CommonReference::read() opening File for G2 from P")
            ).expect("CommonReference::read() for sg2");
        Self {
            code: c.as_bytes().to_vec(),
            qap: q,
            sg1: g1,
            sg2: g2,
        }        
    }

    // improve for error handling/checking file integrity.
    // writes the CommonReference struct to the files given in the Pathfinder.
    pub fn write<P: AsRef<Path>>(&self, paths: PathFinder<P>) {
        let _qap_file = File::create(paths.qap).expect("CommonReference::write() creating File for QAP from P")
            .write_all(
                to_string(&self.qap).expect("CommonReference::write() parsing QAP as string for writing").as_bytes()
            );
        let _sg1_file = File::create(paths.sg1).expect("CommonReference::write() creating File for SigmaG1")
            .write_all(
                to_string(&self.sg1).expect("CommonReference::write() parsing SigmaG1 as string for writing").as_bytes()
            );
        let _sg2_file = File::create(paths.sg2).expect("CommonReference::write() creating File for SigmaG2")
            .write_all(
                to_string(&self.sg2).expect("CommonReference::write() parsing SigmaG2 as string for writing").as_bytes()
            );
    }

    // used to derive a CommonReference struct without reading from file or generating new values.
    pub fn init(c: Vec<u8>, q: QAP<CoefficientPoly<F>>, g1: SigmaG1<G>, g2: SigmaG2<H>) -> Self{
        Self {
            code: c,
            qap: q,
            sg1: g1,
            sg2: g2,
        }  
    }
}

#[test]
fn test_read_reference() {
    use zksnark::{groth16, field::z251::Z251};
    use std::{io::Write, fs::{File, read_to_string}};
    use serde_json::to_string;

    // expected results are that CommonReference::read() will return qap, sg1 and sg2 as before they were written to File.
    let code = read_to_string("src/tests/files/common/simple.zk").expect("internal test err whilst reading code to string");
    let qap: QAP<CoefficientPoly<Z251>> = ASTParser::try_parse(&code).expect("internal test err: ASTParser::try_parse &code unwrapped").into();
    let (sg1, sg2) = groth16::setup(&qap);
    let _qap_file = File::create("src/tests/files/common/common_test.qap").expect("internal test err whilst creating .qap file")
        .write_all(
            to_string(&qap).expect("internal test err whilst reading &qap to string to write to .qap file").as_bytes()
        );
    let _sg1_file = File::create("src/tests/files/common/common_test.sg1").expect("internal test err whilst creating .sg1 file")
        .write_all(
            to_string(&sg1).expect("internal test err whilst reading &sg1 to string to write to .sg1 file").as_bytes()
        );
    let _sg2_file = File::create("src/tests/files/common/common_test.sg2").expect("internal test err whilst reading &sg2 to string to write to .sg2 file")
        .write_all(
            to_string(&sg2).expect("internal test err whilst reading &sg2 to string to write to .sg2 file").as_bytes()
        );
    let crs = CommonReference::<Z251, Z251, Z251>::read(
        PathFinder::<&Path> {
            code: Path::new("src/tests/files/common/simple.zk"),
            qap: Path::new("src/tests/files/common/common_test.qap"),
            sg1: Path::new("src/tests/files/common/common_test.sg1"),
            sg2: Path::new("src/tests/files/common/common_test.sg2"),
        }
    );
    match crs.code == code.as_bytes().to_vec() {
        true => {},
        false => panic!("CommonReference: crs.code != code"),
    };
    match to_string(&crs.qap).unwrap() == to_string(&qap).unwrap() {
        true => {},
        false => panic!("CommonReference: crs.qap != qap"),
    };
    match to_string(&crs.sg1).unwrap() == to_string(&sg1).unwrap() {
        true => {},
        false => panic!("CommonReference: crs.sg1 != sg1"),
    }; 
    match to_string(&crs.sg2).unwrap() == to_string(&sg2).unwrap() {
        true => {},
        false => panic!("CommonReference: crs.sg2 != sg2"),
    };
}
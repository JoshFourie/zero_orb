use std::path::Path;
use zksnark::{SigmaG1, SigmaG2, CoefficientPoly, QAP, 
    field::Field,
    groth16::{
        EllipticEncryptable,
        Random,
        circuit::{ASTParser, TryParse}
    }        
};
use crate::field::{WrappedQAP, WrappedDummyRep};
use std::{str::FromStr, fs::read_to_string};

pub struct CommonReference<F: Field + EllipticEncryptable> {
    pub code: Vec<u8>,
    pub qap: QAP<CoefficientPoly<F>>,
    pub sg1: SigmaG1<<F as EllipticEncryptable>::G1>,
    pub sg2: SigmaG2<<F as EllipticEncryptable>::G2>,
}

pub struct PathFinder<P> {
    pub code: P,
    pub qap: P,
    pub sg1: P,
    pub sg2: P,
}

pub trait Commoner {
    // update paths to tag.
    fn read<'de, P: AsRef<Path>>(paths: PathFinder<P>) -> Self;
}

impl<F> Commoner for CommonReference<F> 
where
    F: Field
    + From<usize>
    + FromStr
    + EllipticEncryptable
    + Random,
{
    fn read<'de, P: AsRef<Path>>(paths: PathFinder<P>) -> Self {
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
}
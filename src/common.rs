use std::path::Path;
use zksnark::{SigmaG1, SigmaG2, CoefficientPoly, QAP,};

pub struct CommonReference<F> {
    pub code: Vec<u8>,
    pub qap: QAP<CoefficientPoly<F>>,
    pub sg1: SigmaG1<F>,
    pub sg2: SigmaG2<F>,
}

pub struct PathFinder<P> {
    pub code: P,
    pub qap: P,
    pub sg1: P,
    pub sg2: P,
}

pub trait Commoner<F> {
    // update paths to tag.
    fn read<'de, P: AsRef<Path>>(paths: PathFinder<P>) -> Self;
}

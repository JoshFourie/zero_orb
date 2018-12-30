use zksnark::groth16::QAP;
use serde::{Serialize, Deserialize};

#[serde(remote = "QAP")]
pub struct QAPDef<P> {
    #[serde(getter = "QAP::CoefficientPoly<Z251>")]
    u: Vec<P>,
    #[serde(getter = "QAP::CoefficientPoly<Z251>")]
    v: Vec<P>,
    #[serde(getter = "QAP::CoefficientPoly<Z251>")]
    w: Vec<P>,
    #[serde(getter = "QAP::CoefficientPoly<Z251>")]
    t: P,
    #[serde(getter = "QAP::CoefficientPoly<Z251>")]
    input: usize,
    #[serde(getter = "QAP::CoefficientPoly<Z251>")]
    degree: usize
}

use crate::{
    common::Common,
    transform::into_field::IntoField,
};
use zksnark::{
    Proof, CoefficientPoly,
    field::Field,
    groth16,
    groth16::{
        EllipticEncryptable,
        Random,
    },
};
use std::{
    str::FromStr,
    ops::{Add, Sub},
    iter::Sum,
};
use serde_derive::{Serialize, Deserialize};

pub trait zkProof {
    fn new<C, T, U, V>(self, crs: C) -> Proof<U, V> 
    where
        C: Common<T, U, V>,
        T: EllipticEncryptable<G1 = U, G2 = V> 
            + Random 
            + Field 
            + Copy 
            + PartialEq
            + From<usize>
            + FromStr,
        U: Add<Output=U> + Sub<Output=U> + Sum + Copy,
        V: Add<Output=V> + Sum + Copy;
}  

pub trait zkVerify {
    fn check<C, T, U, V, W>(self, crs: C, prf: Proof<U, V>) -> bool 
    where 
        C: Common<T, U, V>,
        T: Field 
            + Copy 
            + From<usize>
            + EllipticEncryptable<G1 = U, G2 = V, GT = W>,
        U: Sum,
        V: Add<Output=V> + Sum + Copy,
        W: Add<Output = W> + PartialEq, ;
}

#[derive(Serialize, Deserialize)]
pub struct Knowledge {
    pub wb: Vec<usize>,
    pub vb: Vec<usize>,
    pub wn: Vec<usize>,
    pub vn: Vec<usize>,
    pub ut: String,
}

impl zkProof for Knowledge {
    fn new<C, T, U, V>(self, crs: C) -> Proof<U, V> 
    where
        C: Common<T, U, V>,
        T: EllipticEncryptable<G1 = U, G2 = V> 
            + Random 
            + Field 
            + Copy 
            + PartialEq
            + From<usize>
            + FromStr,
        U: Add<Output=U> + Sub<Output=U> + Sum + Copy,
        V: Add<Output=V> + Sum + Copy,
    {    
        let mut assignments = Vec::new();
        match self.wb.collect_bits(&self.ut) {
            Some(mut x) => assignments.append(&mut x),
            _ => {},
        };
        match self.vb.collect_bits(&self.ut) {
            Some(mut x) => assignments.append(&mut x),
            _ => {},
        };
        match self.wn.collect_nums() {
            Some(mut x) => assignments.append(&mut x),
            _ => {},
        };
        match self.vn.collect_nums() {
            Some(mut x) => assignments.append(&mut x),
            _ => {}
        };
        let (code, qap, sg1, sg2) = crs.get();
        let weights = groth16::weights(&code, &assignments).expect("groth16::weights");    
        groth16::prove(
            &qap,
            (&sg1, &sg2),
            &weights
        )
    }
}

impl Knowledge {
    pub fn init(
        wb: Vec<usize>, vb: Vec<usize>, wn: Vec<usize>, vn: Vec<usize>, ut: String
    ) -> Knowledge {
        Self {
            wb: wb,
            vb: vb,
            wn: wn,
            vn: vn,
            ut: ut,
        }
    }
}

pub struct Marker {
    pub vn: Vec<usize>,
    pub vb: Vec<usize>,
    pub ut: String,
}

impl zkVerify for Marker {
    fn check<C, T, U, V, W>(self, crs: C, prf: Proof<U, V>) -> bool 
    where 
        C: Common<T, U, V>,
        T: Field 
            + From<usize> 
            + Copy 
            + EllipticEncryptable<G1 = U, G2 = V, GT = W>,
        U: Sum,
        V: Add<Output=V> 
            + Sum 
            + Copy,
        W: Add<Output = W> 
            + PartialEq, 
    {
        let mut inputs: Vec<T> = Vec::new();
        match self.vn.collect_nums() {
            Some(mut x) => inputs.append(&mut x),
            _ => {}
        };
        match self.vb.collect_bits(&self.ut) {
            Some(mut x) => inputs.append(&mut x),
            _ => {}
        };
        let (_, _, sg1, sg2) = crs.get();
        groth16::verify::<CoefficientPoly<T>, _, _, _, _>(
            (sg1, sg2),
            &inputs,
            prf
        )
    }
}

#[cfg(test)]
mod tests {
    use zksnark::{
        groth16::{
            fr::{
                FrLocal, G1Local, G2Local, GtLocal
            },
            Proof, QAP, SigmaG1, SigmaG2,
        },
        CoefficientPoly,
    };
    use crate::{
        knowledge::{
            Knowledge, Marker, zkProof, zkVerify
        },
        common::{CommonReference, Common},
    };
    use std::fs::read_to_string;
    use serde_json::{from_str, to_string};


    #[test]
    fn test_simple_num() {
        // x = abc + d + e where a = wn, b = wn, c = vn, d = vn and e = vn
        // verify 18 = (9) (2)
        // enclosure for convenience to build a proof.
        let gen = |a, b: usize| -> Proof<G1Local, G2Local> {
            let k = Knowledge {
                wb: Vec::new(),
                wn: vec![a, b],
                vn: Vec::new(),
                vb: Vec::new(),
                ut: "".to_string(),
            };
            let _crs = CommonReference {
                code: read_to_string("src/tests/files/simple/simple.zk").expect("internal_test: reading code to string"),
                qap: from_str::<QAP<CoefficientPoly<FrLocal>>>(
                    &read_to_string("src/tests/files/simple/simple.qap")
                        .expect("internal_test: reading QAP to string")
                ).expect("internal_test: parsing QAP from string"),
                sg1: from_str::<SigmaG1<G1Local>>(
                    &read_to_string("src/tests/files/simple/simple.sg1")
                        .expect("internal_test: reading SigmaG1 to string")
                ).expect("internal_test: parsing SigmaG1 from string"),
                sg2: from_str::<SigmaG2<G2Local>>(
                    &read_to_string("src/tests/files/simple/simple.sg2")
                        .expect("internal_test: reading SigmaG2 to string")
                ).expect("internal_test: parsing SigmaG2 from string"),
            };
            let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::read(&to_string(&_crs).unwrap());
            k.new(crs)
        };
        //  enclosure for convenience for checking a proof.
        let check = |a: usize, k: Proof<G1Local, G2Local>| -> bool {
            let m = Marker {
                vb: Vec::new(),
                vn: vec![a],
                ut: "".to_string(),
            };
            let _crs = CommonReference {
                code: read_to_string("src/tests/files/simple/simple.zk").expect("internal_test: reading code to string"),
                qap: from_str::<QAP<CoefficientPoly<FrLocal>>>(
                    &read_to_string("src/tests/files/simple/simple.qap")
                        .expect("internal_test: reading QAP to string")
                ).expect("internal_test: parsing QAP from string"),
                sg1: from_str::<SigmaG1<G1Local>>(
                    &read_to_string("src/tests/files/simple/simple.sg1")
                        .expect("internal_test: reading SigmaG1 to string")
                ).expect("internal_test: parsing SigmaG1 from string"),
                sg2: from_str::<SigmaG2<G2Local>>(
                    &read_to_string("src/tests/files/simple/simple.sg2")
                        .expect("internal_test: reading SigmaG2 to string")
                ).expect("internal_test: parsing SigmaG2 from string"),
            };
            let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::read(&to_string(&_crs).unwrap());
            m.check(crs, k)
        };
        // 3 x 2 = 6.
        assert_eq!(
            true,
            check(6, gen(3, 2))
        );  
        // 3 x 2 != 7.
        assert_eq!(
            false,
            check(7, gen(3, 2))
        );    
        // 1 x 2 != 2.
        assert_eq!(
            false,
            check(6, gen(1, 2))
        );      
    }
}
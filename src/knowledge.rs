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
    wb: Option<Vec<usize>>,
    vb: Option<Vec<usize>>,
    wn: Option<Vec<usize>>,
    vn: Option<Vec<usize>>,
    ut: Option<String>,
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
        let mut assignments: Vec<T> = Vec::new();
        match self.wb {
            Some(vec) => {
                match self.ut.clone() {
                    Some(tag) => assignments.append(&mut vec.collect_bits(&tag)),
                    None => {},
                };
            },
            None => {}, 
        }
        match self.vb {
            Some(vec) => {
                match self.ut {
                    Some(tag) => assignments.append(&mut vec.collect_bits(&tag)),
                    None => {},
                };
            },
            None => {}, 
        }
        match self.wn {
            Some(vec) => assignments.append(&mut vec.collect_nums()),
            None => {}, 
        }
        match self.vn {
            Some(vec) => assignments.append(&mut vec.collect_nums()),
            None => {}, 
        }
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
    pub fn into(
        wb: Option<Vec<usize>>, 
        vb: Option<Vec<usize>>, 
        wn: Option<Vec<usize>>, 
        vn: Option<Vec<usize>>, 
        ut: Option<String>
    ) -> Knowledge {
        Self {
            wb: wb,
            vb: vb,
            wn: wn,
            vn: vn,
            ut: ut,
        }
    }
    pub fn into_num_only(wn: Option<Vec<usize>>, vn: Option<Vec<usize>>) -> Self {
        Self::into(
            None, None, wn, vn, None
        )
    }
}

pub struct Marker {
    vn: Option<Vec<usize>>,
    vb: Option<Vec<usize>>,
    ut: Option<String>,
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
        match self.vn {
            Some(vec) => inputs.append(&mut vec.collect_nums()),
            None => {}, 
        }
        match self.vb {
            Some(vec) => {
                match self.ut {
                    Some(tag) => inputs.append(&mut vec.collect_bits(&tag)),
                    None => {},
                };
            },
            None => {}, 
        }
        let (_, _, sg1, sg2) = crs.get();
        groth16::verify::<CoefficientPoly<T>, _, _, _, _>(
            (sg1, sg2),
            &inputs,
            prf
        )
    }
}

impl Marker {
    pub fn into(
        vn: Option<Vec<usize>>,
        vb: Option<Vec<usize>>,
        ut: Option<String>,
    ) -> Self {
        Marker {
            vn: vn,
            vb: vb,
            ut: ut,
        }
    }
}

#[cfg(test)]
mod tests {
    use zksnark::{
        groth16::{
            fr::{
                FrLocal, G1Local, G2Local
            },
            Proof, 
        },
    };
    use crate::{
        knowledge::{
            Knowledge, Marker, zkProof, zkVerify
        },
        common::{CommonReference, Common},
    };
    use std::fs::read_to_string;

    #[test]
    fn test_simple_num() {
        // x = abc + d + e where a = wn, b = wn, c = vn, d = vn and e = vn
        // verify 18 = (9) (2)
        // enclosure for convenience to build a proof.
        let gen = |a, b: usize| -> Proof<G1Local, G2Local> {
            let k = Knowledge {
                wb: None,
                wn: Some(vec![a, b]),
                vn: None,
                vb: None,
                ut: None,
            };
            let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::read(
                &read_to_string("src/tests/files/crs/sample.crs").unwrap()
            );
            k.new(crs)
        };
        //  enclosure for convenience for checking a proof.
        let check = |a: usize, k: Proof<G1Local, G2Local>| -> bool {
            let m = Marker {
                vb: None,
                vn: Some(vec![a]),
                ut: None,
            };
            let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::read(
                &read_to_string("src/tests/files/crs/sample.crs").unwrap()
            );
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
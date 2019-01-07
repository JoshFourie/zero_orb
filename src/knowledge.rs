use crate::{
    common::CommonReference,
    transform::into_field::IntoField,
};
use zksnark::{
    Proof,
    CoefficientPoly,
    field::Field,
    groth16,
    groth16::EllipticEncryptable
};
use std::{
    str::FromStr,
    ops::{Add, Sub},
    iter::Sum,
};
use num::PrimInt;
use serde::{Serialize, Deserialize};  
use serde_derive::{Serialize, Deserialize};

//  MODULE FOR CREATING AND VERIFYING A PROOF.

pub trait zkProof<T, V, W> {
    fn new(self, crs: CommonReference<T, V, W>) -> Proof<V, W>;
}  

pub trait zkVerify<U, R, S> {
    fn check(self, crs: CommonReference<U, R, S>, prf: Proof<R, S>) -> bool;
}

//  Knowledge struct holds 'witness bits', 'variable bits, 'witness num' and 'variable num' that are parseable values for groth16.
// K represents any u-value: u8, u16 etc. P is a placeholder for Paths.
// wb .. vn is constructed like this as the variables should be fed in witness bits -> witness num -> var num -> var bits in the (in of the .zk).
#[derive(Serialize, Deserialize)]
pub struct Knowledge<K> {
    pub wb: Vec<K>,
    pub vb: Vec<K>,
    pub wn: Vec<K>,
    pub vn: Vec<K>,
    pub ut: String,
}

// impl to derive the 'new' and function for the Knowledge struct which builds a Proof object.
// Knowledge can hold any u value provided it is consistent through the struct.
// K --> u16 etc which are all PrimInts from the Num crate, P is a Path. 
impl<'de, K, T, V, W> zkProof<T, V, W> for Knowledge<K>
where
    K: PrimInt,
    T: EllipticEncryptable<G1 = V, G2 = W>
            + Field
            + From<usize>
            + FromStr
            + groth16::Random
            + From<K>
            + Serialize 
            + Deserialize<'de>,
    V: Add<Output=V> + Sub<Output=V> + Sum + Copy + Serialize + Deserialize<'de>,
    W: Add<Output=W> + Sum + Copy + Serialize + Deserialize<'de>,
{
    // builds a proof from the provided values using .zk program pulled from the Paths.
    // takes T: any field e.g. Z655, Z251 etc. , V: SigmaG1<FIELD> type, W: SigmaG2<FIELD> type.
    // returns the groth16::Proof struct which takes G1 and G2 as type arguments.
    // asssignments holds a vec of fields that can be parsed by the groth16 weights argument.
    // appends witness bits/nums and variable bits/nums only where values are present in the Knowledge struct.
    // TODO: replace Vec::new() with alternative to reduce load on heap-mem.
    // See Transform mod for methods
    fn new(self, crs: CommonReference<T, V, W>) -> Proof<V, W> {    
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
        // generates the 'weights' for the zkSNARK.
        let weights = groth16::weights(&crs.code, &assignments).expect("groth16::weights");    
        // builds the proof returned from the function.
        // T = field e.g FrLocal, V = G1 e.g. G1Local or Z251 as EllipticEncryptable, W = G2 e.g. G2Local or Z251 as EllipticEncryptable.
        groth16::prove::<CoefficientPoly<T>, T, V, W>(
            &crs.qap,
            (&crs.sg1, &crs.sg2),
            &weights
        )
    }
}

// 'Marker' holds the verification values as verification bits and verification nums.
pub struct Marker<L> {
    pub vn: Vec<L>,
    pub vb: Vec<L>,
    pub ut: String,
}

// impl for the 'check' which is just a verification of a proof that can be called by the prover/verifier.
// takes L: PrimInt --> u16, u8 etc... for the ^^^ verification values, P for the crs struct.
impl<'de, L, U, R, S> zkVerify<U, R, S> for Marker<L>
where
    L: PrimInt,
    U: EllipticEncryptable<G1 = R, G2 = S>
            + Field
            + From<usize>
            + FromStr
            + groth16::Random
            + From<L>
            + Serialize 
            + Deserialize<'de>,
    R: Add<Output=R> + Sub<Output=R> + Sum + Copy + Serialize + Deserialize<'de>,
    S: Add<Output=S> + Sum + Copy + Serialize + Deserialize<'de>,
    
{
    // check takes R == G1, S == G2, T == GT, U == field e.g. Z655, Z251 etc...
    // returns bool whether proof is correct. 
    fn check(self, crs: CommonReference<U, R, S>, prf: Proof<R,S>) -> bool {
        // stores verification values as fields that are parseable with groth16::verify. 
        // See Transform mod for methods.
        let mut inputs: Vec<U> = Vec::new();
        match self.vn.collect_nums() {
            Some(mut x) => inputs.append(&mut x),
            _ => {}
        };
        match self.vb.collect_bits(&self.ut) {
            Some(mut x) => inputs.append(&mut x),
            _ => {}
        };

        // checks whether a proof is correct and returns a bool.
        groth16::verify::<CoefficientPoly<U>, _, _, _, _>(
            (crs.sg1, crs.sg2),
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
            let k = Knowledge::<usize> {
                wb: Vec::new(),
                wn: vec![a, b],
                vn: Vec::new(),
                vb: Vec::new(),
                ut: "".to_string(),
            };
            let _crs = CommonReference {
                code: read_to_string("src/tests/files/knowledge/simple.zk").expect("internal_test: reading code to string"),
                qap: from_str::<QAP<CoefficientPoly<FrLocal>>>(
                    &read_to_string("src/tests/files/knowledge/simple.qap")
                        .expect("internal_test: reading QAP to string")
                ).expect("internal_test: parsing QAP from string"),
                sg1: from_str::<SigmaG1<G1Local>>(
                    &read_to_string("src/tests/files/knowledge/simple.sg1")
                        .expect("internal_test: reading SigmaG1 to string")
                ).expect("internal_test: parsing SigmaG1 from string"),
                sg2: from_str::<SigmaG2<G2Local>>(
                    &read_to_string("src/tests/files/knowledge/simple.sg2")
                        .expect("internal_test: reading SigmaG2 to string")
                ).expect("internal_test: parsing SigmaG2 from string"),
            };
            let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::read(&to_string(&_crs).unwrap());
            k.new(crs)
        };
        //  enclosure for convenience for checking a proof.
        let check = |a: usize, k: Proof<G1Local, G2Local>| -> bool {
            let m = Marker::<usize> {
                vb: Vec::new(),
                vn: vec![a],
                ut: "".to_string(),
            };
            let _crs = CommonReference {
                code: read_to_string("src/tests/files/knowledge/simple.zk").expect("internal_test: reading code to string"),
                qap: from_str::<QAP<CoefficientPoly<FrLocal>>>(
                    &read_to_string("src/tests/files/knowledge/simple.qap")
                        .expect("internal_test: reading QAP to string")
                ).expect("internal_test: parsing QAP from string"),
                sg1: from_str::<SigmaG1<G1Local>>(
                    &read_to_string("src/tests/files/knowledge/simple.sg1")
                        .expect("internal_test: reading SigmaG1 to string")
                ).expect("internal_test: parsing SigmaG1 from string"),
                sg2: from_str::<SigmaG2<G2Local>>(
                    &read_to_string("src/tests/files/knowledge/simple.sg2")
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

    #[test]
    fn test_zkProof_zkVerify_wrap_unwrap() {

    }
}
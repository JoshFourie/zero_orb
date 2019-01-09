use serde::{Serialize, Deserialize}; 
use serde_derive::{Serialize, Deserialize};
use serde_json::{from_str, to_string};
use zksnark::{
    Proof,
    CoefficientPoly,
    field::Field,
    groth16,
    groth16::fr::{
        FrLocal, 
        G1Local, 
        G2Local,
    },
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
use crate::{
    knowledge::{Knowledge, zkProof},
    common::{CommonReference, Common},
    };

pub trait Transportable {
    fn wrap_as_str(&self) -> String;
    fn unwrap_from_str(m: String) -> Self; 
}

pub trait InterOperable {
    type Operator: Transportable;
    fn go(self) -> Self::Operator;
}

#[derive(Serialize, Deserialize)]
pub struct Andromeda<A, B> {
    crs: A,
    weights: B,
}

impl<A, B> Andromeda<A, B> 
where 
    A: Common<FrLocal, G1Local, G2Local>,
    B: zkProof,
{
    pub fn init(crs: A, weights: B) -> Andromeda<A, B> {
        Self {
            crs: crs,
            weights: weights,
        }
    }
}

impl<A, B> InterOperable for Andromeda<A, B> 
where
    A: Common<FrLocal, G1Local, G2Local>,
    B: zkProof,
{  
    type Operator = BackPack<G1Local, G2Local>;
    fn go(self) -> Self::Operator {
        BackPack {
            prf: self.weights.new(self.crs),
            ver: Vec::new(),
            tag: "Kill ME".to_string(),
            sig: b"Sign that Bad Boi".to_vec(),
            puk: b"Witness Me".to_vec()
        }
    }
}

impl<A, B> Transportable for Andromeda<A, B> 
where
    for <'de>
    A: Common<FrLocal, G1Local, G2Local> 
        + Serialize
        + Deserialize<'de>,
    for <'de>
    B: zkProof 
        + Serialize
        + Deserialize<'de>,
{
    fn wrap_as_str(&self) -> String {
        use serde_json::to_string;
        match to_string(self) {
            Ok(s) => s,
            Err(e) => panic!("Andromeda::wrap_as_str() parsing &self as string: {}", e),
        }
    }
    fn unwrap_from_str(m: String) -> Self{
        use serde_json::from_str;
        match from_str::<Self>(&m) {
            Ok(s) => s,
            Err(e) => panic!("Andromeda::unwrap_from_str() parsing m: N from String: {}", e)
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BackPack<V, W> {
    prf: Proof<V, W>,
    ver: Vec<usize>,
    tag: String,
    sig: Vec<u8>,
    puk: Vec<u8>,    
}

impl<V, W> Transportable for BackPack<V, W> 
where
    for <'de>
    V: Add<Output=V> 
        + Sub<Output=V> 
        + Sum 
        + Copy 
        + Serialize 
        + Deserialize<'de>,
    for <'de>
    W: Add<Output=W> 
        + Sum 
        + Copy 
        + Serialize 
        + Deserialize<'de>,
{
    fn wrap_as_str(&self) -> String {
        use serde_json::to_string;
        match to_string(self) {
            Ok(s) => s,
            Err(e) => panic!("BackPack::wrap_as_str() parsing &self as string: {}", e),
        }
    }
    fn unwrap_from_str(m: String) -> Self{
        use serde_json::from_str;
        match from_str::<Self>(&m) {
            Ok(s) => s,
            Err(e) => panic!("BackPack::unwrap_from_str() parsing m: N from String: {}", e)
        }
    }
}

#[cfg(test)]
mod test {
    use zksnark::{
        groth16::{
            fr::{
                FrLocal, G1Local, G2Local, GtLocal
            },
            Proof, QAP, SigmaG1, SigmaG2,
        },
        CoefficientPoly,
    };
    use std::fs::read_to_string;
    use serde_json::{from_str, to_string};
    use crate::{
        common::{
            CommonReference, 
            Common, 
            RefFinder
        },
        knowledge::{Knowledge, Marker, zkVerify},
        interface::{Andromeda, InterOperable},
    };

    fn quick_get_crs() -> CommonReference<FrLocal, G1Local, G2Local> {
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
        crs
    }

    #[test]
    fn test_andromeda_parsing() {
        let crs = quick_get_crs();
        let weights = Knowledge {
            wb: Vec::new(),
            wn: vec![20, 5],
            vn: Vec::new(),
            vb: Vec::new(),
            ut: "".to_string(),
        };
        let andromeda = Andromeda {
            weights: weights,
            crs: crs,
        };
        let x = andromeda.go();
        let m = Marker {
            vn: vec![100],
            vb: Vec::new(),
            ut: "".to_string(),
        };
        let crs = quick_get_crs();
        let b = m.check(crs, x.prf);
        assert!(b);
    }
}
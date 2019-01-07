use serde::{Serialize, Deserialize}; 
use serde_derive::{Serialize, Deserialize};
use zksnark::{
    Proof,
    CoefficientPoly,
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
use crate::{
    knowledge::{Knowledge, zkProof},
    common::{CommonReference, Common},
    };
use num::PrimInt;

pub trait Transportable {
    fn wrap_as_str(&self) -> String;
    fn unwrap_from_str(m: String) -> Self; 
}

#[derive(Serialize, Deserialize)]
pub struct Backpack<V, W> {
    prf: Proof<V, W>,
    ver: Vec<u8>,
    tag: String,
    sig: Vec<u8>,
    puk: Vec<u8>,    
}


#[derive(Serialize, Deserialize)]
pub struct Andromeda<T, U> {
    weights: T,
    crs: U,
}

impl<V, W> Transportable for Backpack<V, W> 
where
    for <'de> 
    V: Add<Output=V> + Sub<Output=V> + Sum + Copy + Serialize + Deserialize<'de>,
    for <'de> 
    W: Add<Output=W> + Sum + Copy + Serialize + Deserialize<'de>,
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

// impl<T, U, X, Y, Z> zkProof<X, Y, Z> for Andromeda<T, U> {
   // fn new(self, crs: CommonReference<T, V, W>) -> Proof<V, W>;
// }

impl<T, U> Transportable for Andromeda<T, U>
where
    for <'de> 
    T: Serialize 
        + Deserialize<'de>,
    for <'de> 
    U: Common
        + Serialize 
        + Deserialize<'de>
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

impl<K> Transportable for Knowledge<K> 
where   
    for <'de> 
    K: PrimInt + Serialize + Deserialize<'de>
{
    fn wrap_as_str(&self) -> String {
        use serde_json::to_string;
        match to_string(self) {
            Ok(s) => s,
            Err(e) => panic!("Knowledge::wrap_as_str() parsing &self as string: {}", e),
        }
    }
    fn unwrap_from_str(m: String) -> Self{
        use serde_json::from_str;
        match from_str::<Self>(&m) {
            Ok(s) => s,
            Err(e) => panic!("Knowledge::unwrap_from_str() parsing m: N from String: {}", e)
        }
    }
}

impl<F, G, H> Transportable for CommonReference<F, G, H> 
where
    for <'de>
    F: Serialize + Deserialize<'de>,
    for <'de>
    G: Serialize + Deserialize<'de>,
    for <'de>
    H: Serialize + Deserialize<'de>,
{
    fn wrap_as_str(&self) -> String {
        use serde_json::to_string;
        match to_string(self) {
            Ok(s) => s,
            Err(e) => panic!("CommonReference::wrap_as_str() parsing &self as string: {}", e),
        }
    }
    fn unwrap_from_str(m: String) -> Self{
        use serde_json::from_str;
        match from_str::<Self>(&m) {
            Ok(s) => s,
            Err(e) => panic!("CommonReference::unwrap_from_str() parsing m: N from String: {}", e)
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
            Common
        },
        knowledge::{Knowledge},
        interface::Andromeda,
    };

    fn quick_get_crs() -> CommonReference<FrLocal, G1Local, G2Local> {
        let _crs = CommonReference {
            code: read_to_string("src/tests/files/interface/simple.zk").expect("internal_test: reading code to string"),
            qap: from_str::<QAP<CoefficientPoly<FrLocal>>>(
                &read_to_string("src/tests/files/interface/simple.qap")
                    .expect("internal_test: reading QAP to string")
            ).expect("internal_test: parsing QAP from string"),
            sg1: from_str::<SigmaG1<G1Local>>(
                &read_to_string("src/tests/files/interface/simple.sg1")
                    .expect("internal_test: reading SigmaG1 to string")
            ).expect("internal_test: parsing SigmaG1 from string"),
            sg2: from_str::<SigmaG2<G2Local>>(
                &read_to_string("src/tests/files/interface/simple.sg2")
                    .expect("internal_test: reading SigmaG2 to string")
            ).expect("internal_test: parsing SigmaG2 from string"),
        };
        let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::read(&to_string(&_crs).unwrap());
        crs
    }

    #[test]
    fn test_andromeda_parsing() {
        let crs = quick_get_crs();
        let weights = Knowledge::<usize> {
            wb: Vec::new(),
            wn: vec![20, 5],
            vn: Vec::new(),
            vb: Vec::new(),
            ut: "".to_string(),
        };
        let _andromeda = Andromeda {
            weights: weights,
            crs: crs,
        };
    }
}
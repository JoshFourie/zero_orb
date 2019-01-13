use zksnark::{
    Proof, 
    field::Field,
    groth16::{
        EllipticEncryptable,
        Random,
    },
};
use std::{
    str::FromStr,
    ops::{Add, Sub},
    iter::Sum,
    marker::PhantomData,
};
use ring::{
    signature::{
        Ed25519KeyPair, 
        KeyPair
        },
};
use crate::{
    knowledge::zkProof,
    common::Common,
    crypto::{EdDSA, SignatureScheme},
};
use serde_derive::{Serialize, Deserialize};
use serde::{Serialize, Deserialize};

pub trait GoZero<'de> {
    type Returner: Serialize + Deserialize<'de>;
    fn go(self) -> Self::Returner;
}

pub trait MarkZero {   
    fn verify(self) -> bool;
}

pub struct Andromeda<A, B, T, U, V, W> {
    crs: A,
    weights: B,
    compute_out: Option<Vec<usize>>,
    key_pair: Ed25519KeyPair,
    _phantom_fr: PhantomData<T>,
    _phantom_g1: PhantomData<U>,
    _phantom_g2: PhantomData<V>,
    _phantom_gt: PhantomData<W>, 
}

#[derive(Serialize, Deserialize)]
pub struct BackPack<A, T, U, V, W> {
    prf: Proof<U, V>,
    ver: Option<Vec<usize>>,
    tag: String,
    sig: Vec<u8>,
    puk: Vec<u8>,
    crs: A,
    _phantom_fr: PhantomData<T>,
    _phantom_gt: PhantomData<W>,
}

impl<A, T, U, V, W> BackPack<A, T, U, V, W> {
    pub fn into(
        prf: Proof<U, V>,
        ver: Option<Vec<usize>>,
        tag: String,
        sig: Vec<u8>,
        puk: Vec<u8>,    
        crs: A,
    ) -> Self {
        BackPack {
            prf: prf,
            ver: ver,
            tag: tag,
            sig: sig,
            puk: puk,    
            crs: crs,
            _phantom_fr: PhantomData::<T>,
            _phantom_gt: PhantomData::<W>,
        }
    }
}

impl<A, B, T, U, V, W> Andromeda<A, B, T, U, V, W> {
    pub fn into(
        crs: A, 
        weights: B, 
        compute_out: Option<Vec<usize>>,
        key_pair: Ed25519KeyPair
    ) -> Andromeda<A, B, T, U, V, W> {
        Self {
            crs: crs,
            weights: weights,
            key_pair: key_pair,
            compute_out: compute_out,
            _phantom_fr: PhantomData::<T>,
            _phantom_g1: PhantomData::<U>,
            _phantom_g2: PhantomData::<V>,
            _phantom_gt: PhantomData::<W>, 
        }
    }
}

impl<'de, A, B, T, U, V, W> GoZero<'de> for Andromeda<A, B, T, U, V, W> 
where
    A: Common<T, U, V>
        + Serialize
        + Deserialize<'de>,
    B: zkProof,
    T: Field 
        + From<usize> 
        + Copy 
        + EllipticEncryptable<G1 = U, G2 = V, GT = W>
        + Random
        + FromStr
        + Serialize
        + Deserialize<'de>,
    U: Add<Output=U>
        + Sum
        + Sub<Output=U> 
        + Copy
        + Serialize
        + Deserialize<'de>,
    V: Add<Output=V> 
        + Sum 
        + Copy
        + Serialize
        + Deserialize<'de>,
    W: Add<Output = W> 
        + PartialEq, 
{  
    type Returner = BackPack<A, T, U, V, W>;

    fn go(self) -> Self::Returner {
        use serde_json::to_string;
        let prf = self.weights.new(self.crs.clone());
        let sig = EdDSA::into(
            to_string(&prf)
            .expect("Andromemda::GoZero::go() failed to parse &prf as a string for the EdDSA tuple-struct")   
        ).sign_message(&self.key_pair);
        BackPack::into(
            prf,
            self.compute_out,
            "tag_blank".to_string(),
            sig.as_ref()
                .to_vec(),
            self.key_pair
                .public_key()
                .as_ref()
                .to_vec(),
            self.crs,
        )
    }
}

// Should pull the relevant VN and VB values from a database, but for the interim we'll hardcode this.
impl<A, T, U, V, W> MarkZero for BackPack<A, T, U, V, W> 
where
    A: Common<T, U, V>,
    for <'de>
    T: Field 
        + From<usize> 
        + Copy 
        + EllipticEncryptable<G1 = U, G2 = V, GT = W>
        + Serialize
        + Deserialize<'de>,
    for <'de>
    U: Sum
        + Serialize
        + Deserialize<'de>,
    for <'de>
    V: Add<Output=V> 
        + Sum 
        + Copy
        + Serialize
        + Deserialize<'de>,
    for <'de>
    W: Add<Output = W> 
        + PartialEq, 
{
    fn verify(self) -> bool {
        use serde_json::to_string;
        use crate::{
            knowledge::{Marker, zkVerify},
            crypto::{EdDSA, SignatureScheme},
        };
        match (
            EdDSA::into(
                to_string(&self.prf).unwrap()
            ).verify_signature(
                &self.sig,
                &self.puk
            ),
            Marker::into(
                self.ver,
                None,
                None,
            ).check(
                self.crs,
                self.prf,
            ),
        ) {
            (true, true) => true,
            (_, _) => false
        }
    }
}

#[cfg(test)]
mod test {
    use zksnark::groth16::fr::{
        FrLocal, G1Local, G2Local,
    };
    use std::fs::read_to_string;
    use crate::{
        common::{CommonReference, Common},
        crypto::{EdDSA, SignatureScheme},
        knowledge::Knowledge,
        interface::{GoZero, MarkZero, Andromeda},
    };

    #[test]
    fn test_andromeda_parsing() {
        let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::read(
            &read_to_string("src/tests/files/crs/sample.crs").unwrap()
        );
        let weights = Knowledge::into(
            None, 
            None,
            Some(vec![20, 5]),
            None,
            None,
        );
        assert_eq!(
            true,
            Andromeda::into(
                crs.clone(), 
                weights, 
                Some(vec![100]),
                EdDSA::<String>::init_key_pair()
            ).go().verify()
        );
    }
}
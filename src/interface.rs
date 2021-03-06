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

pub trait ZeroRef {
    type C;
    fn copy_str(&self) -> Self::C;
    fn get_prf_str(&self) -> String;
    fn get_ver_str(&self) -> String;
    fn get_sig_str(&self) -> String;
    fn get_puk_str(&self) -> String;
    fn get_crs_str(&self) -> String;
}

pub trait MarkZero {   
    fn verify(self) -> bool;
}

pub struct Andromeda<A, B, T, U, V, W> {
    crs: A,
    weights: B,
    compute_out: Option<Vec<usize>>,
    key_pair: Box<[u8]>,
    _phantom_fr: PhantomData<T>,
    _phantom_g1: PhantomData<U>,
    _phantom_g2: PhantomData<V>,
    _phantom_gt: PhantomData<W>, 
}

#[derive(Serialize, Deserialize)]
pub struct BackPack<A, T, U, V, W> {
    pub prf: Proof<U, V>,
    pub ver: Option<Vec<usize>>,
    pub sig: Box<[u8]>,
    pub puk: Box<[u8]>,    
    pub crs: A,
    _phantom_fr: PhantomData<T>,
    _phantom_gt: PhantomData<W>,
}

impl<A, B, T, U, V, W> Andromeda<A, B, T, U, V, W> {
    pub fn into(
        crs: A, 
        weights: B, 
        compute_out: Option<Vec<usize>>,
        key_pair: Box<[u8]>,
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

impl<A, T, U, V, W> BackPack<A, T, U, V, W> {
    pub fn into(
        prf: Proof<U, V>,
        ver: Option<Vec<usize>>,
        sig: Box<[u8]>,
        puk: Box<[u8]>,    
        crs: A,
    ) -> Self {
        BackPack {
            prf: prf,
            ver: ver,
            sig: sig,
            puk: puk,    
            crs: crs,
            _phantom_fr: PhantomData::<T>,
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
        let prf = self.weights.new(self.crs.clone());
        let sig = EdDSA::into(
            serde_json::to_string(&prf)
            .expect("Andromemda::GoZero::go() failed to parse &prf as a string for the EdDSA tuple-struct")   
        ).sign_message(&self.key_pair);
        BackPack::into(
            prf,
            self.compute_out,
            sig,
            EdDSA::<String>::public_key(&self.key_pair)
                .as_ref()
                .to_vec()
                .into_boxed_slice(),
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

impl<A, T, U, V, W> ZeroRef for BackPack<A, T, U, V, W>
where
    A: Serialize,
    U: Serialize, 
    V: Serialize,
{
    type C = (String, String, String, String, String);
    
    fn copy_str(&self) -> Self::C {
        return(
            serde_json::to_string(&self.prf)
                .expect("BackPack::ZeroRef::copy::to_string::prf panicked whilst deserializing field prf for BackPack"),
            serde_json::to_string(&self.ver)
                .expect("BackPack::ZeroRef::copy::to_string::ver panicked whilst deserializing field ver for BackPack"),
            serde_json::to_string(&self.sig)
                .expect("BackPack::ZeroRef::copy::to_string::sig panicked whilst deserializing field sig for BackPack"),
            serde_json::to_string(&self.puk)
                .expect("BackPack::ZeroRef::copy::to_string::puk panicked whilst deserializing field puk for BackPack"),
            serde_json::to_string(&self.crs)
                .expect("BackPack::ZeroRef::copy::to_string::crs panicked whilst deserializing field crs for BackPack"),
        )
    }
    fn get_prf_str(&self) -> String {
        serde_json::to_string(&self.prf).expect("BackPack::ZeroRef::get_prf_str::to_string::prf panicked whilst deserializing field prf for BackPack")
    }
    fn get_ver_str(&self) -> String {
        serde_json::to_string(&self.ver).expect("BackPack::ZeroRef::get_ver_str::to_string::ver panicked whilst deserializing field ver for BackPack")
    }
    fn get_sig_str(&self) -> String {
        serde_json::to_string(&self.sig).expect("BackPack::ZeroRef::get_sig_str::to_string::sig panicked whilst deserializing field sig for BackPack")
    }
    fn get_puk_str(&self) -> String {
        serde_json::to_string(&self.puk).expect("BackPack::ZeroRef::get_puk_str::to_string::puk panicked whilst deserializing field puk for BackPack")
    }
    fn get_crs_str(&self) -> String {
        serde_json::to_string(&self.crs).expect("BackPack::ZeroRef::get_crs_str::to_string::crs panicked whilst deserializing field crs for BackPack")
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
        let weights = |a, b: usize| -> Knowledge {
            Knowledge::into(
                None, 
                None,
                Some(vec![a, b]),
                None,
                None,
            )
        };
        assert_eq!(
            true,
            Andromeda::into(
                crs.clone(), 
                weights(20, 5), 
                Some(vec![100]),
                EdDSA::<String>::init_key_pair()
            ).go().verify()
        );
        assert_eq!(
            false,
            Andromeda::into(
                crs.clone(), 
                weights(10, 5), 
                Some(vec![100]),
                EdDSA::<String>::init_key_pair()
            ).go().verify()
        );
        assert_eq!(
            false,
            Andromeda::into(
                crs.clone(), 
                weights(10, 2), 
                Some(vec![100]),
                EdDSA::<String>::init_key_pair()
            ).go().verify()
        );
        assert_eq!(
            false,
            Andromeda::into(
                crs.clone(), 
                weights(20, 5), 
                Some(vec![90]),
                EdDSA::<String>::init_key_pair()
            ).go().verify()
        );
    }
}
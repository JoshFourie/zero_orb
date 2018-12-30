use crate::{
    common::{PathFinder, CommonReference, Commoner},
    transform::IntoField,
};
use zksnark::{
    Proof,
    CoefficientPoly,
    field::Field,
    groth16,
    groth16::EllipticEncryptable
};
use std::{
    path::Path,
    str::FromStr,
    ops::{Add, Sub},
    iter::Sum,
};
use num::PrimInt;  

//  MODULE FOR CREATING AND VERIFYING A PROOF.

//  Knowledge struct holds 'witness bits', 'variable bits, 'witness num' and 'variable num' that are parseable values for groth16.
// K represents any u-value: u8, u16 etc. P is a placeholder for Paths.
pub struct Knowledge<K, P> {
    pub wb: Vec<K>,
    pub vb: Vec<K>,
    pub wn: Vec<K>,
    pub vn: Vec<K>,
    pub t: Vec<u8>,
    pub pth: PathFinder<P>,
}

// impl to derive the 'new' and function for the Knowledge struct which builds a Proof object.
// Knowledge can hold any u value provided it is consistent through the struct.
// K --> u16 etc which are all PrimInts from the Num crate, P is a Path. 
impl<K: PrimInt, P: AsRef<Path>> Knowledge<K, P>
{
    // builds a proof from the provided values using .zk program pulled from the Paths.
    // takes T: any field e.g. Z655, Z251 etc. , V: SigmaG1<FIELD> type, W: SigmaG2<FIELD> type.
    // returns the groth16::Proof struct which takes G1 and G2 as type arguments.
    pub fn new<T, V, W>(self) -> Proof<V, W>
    where 
        T: EllipticEncryptable<G1 = V, G2 = W>
            + Field
            + From<usize>
            + FromStr
            + groth16::Random
            + From<K>,
        V: Add<Output=V> + Sub<Output=V> + Sum + Copy,
        W: Add<Output=W> + Sum + Copy,
    {
        // crs holds a struct that reads the stored QAP, Code and G1, G2 values from file.
        let crs: CommonReference<T> = CommonReference::read(self.pth);
        
        // asssignments holds a vec of fields that can be parsed by the groth16 weights argument.
        // See Transform mod for methods
        let mut assignments = Vec::new();
        match self.wb.collect_bits() {
            Some(mut x) => assignments.append(&mut x),
            _ => {},
        };
        match self.vb.collect_bits() {
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
        let weights = groth16::weights(
            std::str::from_utf8(
                crs.code.as_slice()
            ).expect("from_utf8 for groth16::weights"), 
            &assignments
        ).expect("groth16::weights");    

        // builds the proof returned from the function.
        groth16::prove(
            &crs.qap,
            (&crs.sg1, &crs.sg2),
            weights.as_slice()
        )
    }
}

// 'Marker' holds the verification values as verification bits and verification nums.
pub struct Marker<L, P> {
    pub vn: Vec<L>,
    pub vb: Vec<L>,
    pub pth: PathFinder<P>,
}

// impl for the 'check' which is just a verification of a proof that can be called by the prover/verifier.
// takes L: PrimInt --> u16, u8 etc... for the ^^^ verification values, P for the crs struct.
impl<L: PrimInt, P: AsRef<Path>> Marker<L, P> 
{
    // check takes R == G1, S == G2, T == GT, U == field e.g. Z655, Z251 etc...
    // returns bool whether proof is correct. 
    pub fn check<R, S, T, U>(self, prf: Proof<R,S>) -> bool 
    where
        U: EllipticEncryptable<G1 = R, G2 = S, GT = T>
            + Field
            + From<usize>
            + FromStr
            + groth16::Random
            + From<L>,
        R: Add<Output=R> + Sub<Output=R> + Sum + Copy,
        S: Add<Output=S> + Sum + Copy,
        T: Add<Output=T> + PartialEq,
    {
        // crs generator...
        let crs: CommonReference<U> = CommonReference::read(self.pth);

        // stores verification values as fields that are parseable with groth16::verify. See Transform mod for methods.
        let mut inputs: Vec<U> = Vec::new();
        match self.vn.collect_nums() {
            Some(mut x) => inputs.append(&mut x),
            _ => {}
        };
        match self.vb.collect_bits() {
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
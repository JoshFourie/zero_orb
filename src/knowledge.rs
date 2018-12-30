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

pub struct Knowledge<K, P> {
    pub wb: Vec<K>,
    pub vb: Vec<K>,
    pub wn: Vec<K>,
    pub vn: Vec<K>,
    pub t: Vec<u8>,
    pub pth: PathFinder<P>,
}
   
impl<K: PrimInt, P: AsRef<Path>> Knowledge<K, P>
{
    fn new<T, V, W>(self) -> Proof<V, W>
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
        let crs: CommonReference<T> = CommonReference::read(self.pth);
        let mut assignments = Vec::new();
        assignments.append(&mut self.wb.collect_bits());
        assignments.append(&mut self.vb.collect_bits());
        assignments.append(&mut self.wn.collect_nums());
        assignments.append(&mut self.vn.collect_nums());
        let weights = groth16::weights(
            std::str::from_utf8(
                crs.code.as_slice()
            ).unwrap(), 
            &assignments
        ).unwrap();    
        groth16::prove(
            &crs.qap,
            (&crs.sg1, &crs.sg2),
            weights.as_slice()
        )
    }
}

pub struct Marker<L, P> {
    pub vn: Vec<L>,
    pub vb: Vec<L>,
    pub pth: PathFinder<P>,
}

impl<L: PrimInt, P: AsRef<Path>> Marker<L, P> {
    fn check<R, S, T, U>(self, prf: Proof<R,S>) -> bool 
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
        let crs: CommonReference<U> = CommonReference::read(self.pth);
        let mut inputs: Vec<U> = Vec::new();
        inputs.append(&mut self.vn.collect_nums());
        inputs.append(&mut self.vb.collect_bits());
        groth16::verify::<CoefficientPoly<U>, _, _, _, _>(
            (crs.sg1, crs.sg2),
            &inputs,
            prf
        )
    }
}
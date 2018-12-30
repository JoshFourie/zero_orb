use std::path::Path;
use num::PrimInt;  

pub struct Knowledge<'a, K> {
    pub wb: Vec<K>,
    pub vb: Vec<K>,
    pub wn: Vec<K>,
    pub vn: Vec<K>,
    pub t: Vec<u8>,
    pub p: [&'a Path; 4],
}

pub struct Marker<'a, K: PrimInt> {
    pub vn: Vec<K>,
    pub vb: Vec<K>,
    pub p: [&'a Path; 4],
}

pub trait Knowledgeable<K: PrimInt> {
    fn new(knowledge: Knowledge<K>) -> Self;
    fn check(self, marker: Marker<K>) -> bool;
    fn as_bits(&self) -> Vec<u8>;
}
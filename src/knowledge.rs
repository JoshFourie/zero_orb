use crate::{
    common::{PathFinder, CommonReference},
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
    path::Path,
    str::FromStr,
    ops::{Add, Sub},
    iter::Sum,
};
use num::PrimInt;
use serde::{Serialize, Deserialize};  

//  MODULE FOR CREATING AND VERIFYING A PROOF.

//  Knowledge struct holds 'witness bits', 'variable bits, 'witness num' and 'variable num' that are parseable values for groth16.
// K represents any u-value: u8, u16 etc. P is a placeholder for Paths.
// wb .. vn is constructed like this as the variables should be fed in witness bits -> witness num -> var num -> var bits in the (in of the .zk).
pub struct Knowledge<K> {
    pub wb: Vec<K>,
    pub vb: Vec<K>,
    pub wn: Vec<K>,
    pub vn: Vec<K>,
    pub ut: &'static str,
}

// impl to derive the 'new' and function for the Knowledge struct which builds a Proof object.
// Knowledge can hold any u value provided it is consistent through the struct.
// K --> u16 etc which are all PrimInts from the Num crate, P is a Path. 
impl<K> Knowledge<K>
where
    K: PrimInt + 'static,
{
    // builds a proof from the provided values using .zk program pulled from the Paths.
    // takes T: any field e.g. Z655, Z251 etc. , V: SigmaG1<FIELD> type, W: SigmaG2<FIELD> type.
    // returns the groth16::Proof struct which takes G1 and G2 as type arguments.
    pub fn new<P, T, V, W>(self, crs: CommonReference<T, V, W>) -> Proof<V, W> 
    where
        P: AsRef<Path>,
        for <'de> T: EllipticEncryptable<G1 = V, G2 = W>
            + Field
            + From<usize>
            + FromStr
            + groth16::Random
            + From<K>
            + Serialize 
            + Deserialize<'de>,
        for <'de> V: Add<Output=V> + Sub<Output=V> + Sum + Copy + Serialize + Deserialize<'de>,
        for <'de> W: Add<Output=W> + Sum + Copy + Serialize + Deserialize<'de>,
    {    
        // asssignments holds a vec of fields that can be parsed by the groth16 weights argument.
        // appends witness bits/nums and variable bits/nums only where values are present in the Knowledge struct.
        // TODO: replace Vec::new() with alternative to reduce load on heap-mem.
        // See Transform mod for methods
        let mut assignments = Vec::new();
        match self.wb.collect_bits(self.ut) {
            Some(mut x) => assignments.append(&mut x),
            _ => {},
        };
        match self.vb.collect_bits(self.ut) {
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
        // T = field e.g FrLocal, V = G1 e.g. G1Local or Z251 as EllipticEncryptable, W = G2 e.g. G2Local or Z251 as EllipticEncryptable.
        groth16::prove::<CoefficientPoly<T>, T, V, W>(
            &crs.qap,
            (&crs.sg1, &crs.sg2),
            &weights
        )
    }

    pub fn new_from_file<P, T, V, W>(self, pth: PathFinder<P>) -> Proof<V, W>
    where
        P: AsRef<Path>,
        for <'de> T: EllipticEncryptable<G1 = V, G2 = W>
            + Field
            + From<usize>
            + FromStr
            + groth16::Random
            + From<K>
            + Serialize 
            + Deserialize<'de>,
        for <'de> V: Add<Output=V> + Sub<Output=V> + Sum + Copy + Serialize + Deserialize<'de>,
        for <'de> W: Add<Output=W> + Sum + Copy + Serialize + Deserialize<'de>,
    { 
        let crs: CommonReference<T, V, W> = CommonReference::read(pth);
        Self::new::<P, T, V, W>(self, crs)
    }
}

// 'Marker' holds the verification values as verification bits and verification nums.
pub struct Marker<L, P> {
    pub vn: Vec<L>,
    pub vb: Vec<L>,
    pub ut: &'static str,
    pub pth: PathFinder<P>,
}

// impl for the 'check' which is just a verification of a proof that can be called by the prover/verifier.
// takes L: PrimInt --> u16, u8 etc... for the ^^^ verification values, P for the crs struct.
impl<L: PrimInt + 'static, P: AsRef<Path> + Clone> Marker<L, P> 
{
    // check takes R == G1, S == G2, T == GT, U == field e.g. Z655, Z251 etc...
    // returns bool whether proof is correct. 
    pub fn check<U, R, S, T>(self, prf: Proof<R,S>) -> bool 
    where
        for <'de> U: EllipticEncryptable<G1 = R, G2 = S, GT = T>
            + Field
            + From<usize>
            + FromStr
            + groth16::Random
            + From<L>
            + Serialize 
            + Deserialize<'de>,
        for <'de> R: Add<Output=R> + Sub<Output=R> + Sum + Copy + Serialize + Deserialize<'de>,
        for <'de> S: Add<Output=S> + Sum + Copy + Serialize + Deserialize<'de>,
        for <'de> T: Add<Output=T> + PartialEq + Serialize + Deserialize<'de>,
    {
        // reading the CRS from file.
        // This MUST be IDENTICAL to the CRS used in the Knowledge::new() method.
        let crs: CommonReference<U, R, S> = CommonReference::read(self.pth);

        // stores verification values as fields that are parseable with groth16::verify. 
        // See Transform mod for methods.
        let mut inputs: Vec<U> = Vec::new();
        match self.vn.collect_nums() {
            Some(mut x) => inputs.append(&mut x),
            _ => {}
        };
        match self.vb.collect_bits(self.ut) {
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

#[test]
fn test_simple_num() {
    use zksnark::groth16::fr::{FrLocal, G1Local, G2Local, GtLocal};
    // x = abc + d + e where a = wn, b = wn, c = vn, d = vn and e = vn
    // verify 18 = (9) (2)
    // enclosure for convenience to build a proof.
    let gen = |a, b: usize| -> Proof<G1Local, G2Local> {
        let k = Knowledge::<usize> {
            wb: Vec::new(),
            wn: vec![a, b],
            vn: Vec::new(),
            vb: Vec::new(),
            ut: "",
        };
        let p = PathFinder::<&Path> {
            code: Path::new("src/tests/files/simple.zk"),
            qap: Path::new("src/tests/files/knowledge/simple.qap"),
            sg1: Path::new("src/tests/files/knowledge/simple.sg1"),
            sg2: Path::new("src/tests/files/knowledge/simple.sg2"),
        };
        k.new_from_file::<&Path, FrLocal, G1Local, G2Local>(p)
    };
    //  enclosure for convenience for checking a proof.
    let check = |a: usize, k: Proof<G1Local, G2Local>| -> bool {
        let m = Marker::<usize, &Path> {
            vb: Vec::new(),
            vn: vec![a],
            ut: "",
            pth: PathFinder::<&Path> {
                code: Path::new("src/tests/files/simple.zk"),
                qap: Path::new("src/tests/files/knowledge/simple.qap"),
                sg1: Path::new("src/tests/files/knowledge/simple.sg1"),
                sg2: Path::new("src/tests/files/knowledge/simple.sg2"),
            },
        };
        m.check::<FrLocal, G1Local, G2Local, GtLocal>(k)
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

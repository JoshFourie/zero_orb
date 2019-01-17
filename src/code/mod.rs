#[test]
fn test_AND_gate() {
    use crate::knowledge::{Knowledge, Marker, zkProof, zkVerify};
    use crate::common::{Common, CommonReference};
    use crate::crypto::{EdDSA, SignatureScheme};
    use crate::interface::{Andromeda, GoZero, MarkZero};
    use zksnark::groth16::fr::{FrLocal, G1Local, G2Local};

    // an AND gate is analytically represented as the product of two bits.

    let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::new(
        std::fs::read_to_string("src/code/and.zk").unwrap()
    );
    let weights = Knowledge::into(
        None, 
        None,
        Some(vec![0, 1, 1, 1, 0, 1, 0, 1]),
        None,
        None,
    );
    // we are expecting that running 0111 and 0101 through an AND gate will get us 0101 back.
    assert!(
        Andromeda::into(
            crs.clone(), 
            weights, 
            Some(vec![0, 1, 0, 1]),
            EdDSA::<String>::init_key_pair()
        ).go().verify()
    );
}

#[test]
fn test_OR_gate() {
    use crate::knowledge::{Knowledge, Marker, zkProof, zkVerify};
    use crate::common::{Common, CommonReference};
    use crate::crypto::{EdDSA, SignatureScheme};
    use crate::interface::{Andromeda, GoZero, MarkZero};
    use zksnark::groth16::fr::{FrLocal, G1Local, G2Local};

    // an OR gate is analytically represented as f(a, b) = a + b - ab.

    let crs: CommonReference<FrLocal, G1Local, G2Local> = CommonReference::new(
        std::fs::read_to_string("src/code/or.zk").expect("internal_test: reading CommonReference from string")
    );
    let weights = Knowledge::into(
        None, 
        None,
        Some(vec![0, 1, 0b1001]),
        None,
        None,
    );
    // we are expecting that running 0111 and 0101 through an AND gate will get us 0101 back.
    assert_eq!(
        true,
        Andromeda::into(
            crs.clone(), 
            weights, 
            Some(vec![1]),
            EdDSA::<String>::init_key_pair()
        ).go().verify()
    );
}
use itertools::Itertools;
use num::{PrimInt, NumCast};
use zksnark::field::Field;

// trait for parsing Vec::<PrimInt> into a vec of field values as either bits or usize.
pub trait IntoField<S> {
    fn collect_nums<T>(self) -> Option<Vec<T>>
    where 
        T: Field + From<S>;
    fn collect_bits<U>(self, t: &'static str) -> Option<Vec<U>>
    where
        U: Field + From<S>;
}

// S: PrimInt lets the impl function across u8 -> u64 generically.
// matching the length of the vec and returning a None is currently used to process empty values in the Knowledge struct.
// 'static str solution in fn collect_bits() is required for determining whether a number should be converted to 64, 32 etc bits.
// the string solution is relevant as it enables us to derive bits of a type not necessarily connected to the provided value (within some bounds) e.g. 8 bits for a u32 number.
// TODO: impl guards for asserting a u64 value as 8 bits etc.
impl<S: PrimInt> IntoField<S> for Vec<S> {
    fn collect_nums<T>(self) -> Option<Vec<T>>
    where
        T: Field + From<S>,
    {
        if self.len() == 0 {
            None
        } else {
            Some(
                self.into_iter()
                    .map(|n| {
                        let x = T::from(n);
                        x
                }).collect::<Vec<T>>()
            )
        }
    }
    fn collect_bits<U>(self, t: &'static str) -> Option<Vec<U>> 
    where
        U: Field + From<S>,
    {
        if self.len() == 0 {
            None
        } else {
            let x = self.into_iter()
                .map(|mut n| {
                    let mut bits: Vec<S> = Vec::new(); 
                    let len: usize;
                    match t {
                        "u8" => { 
                            bits.extend_from_slice(&[S::zero(); 8]); 
                            len = 8; 
                        },
                        "u16" => { 
                            bits.extend_from_slice(&[S::zero(); 16]); 
                            len = 16; 
                        },
                        "u32" => { 
                            bits.extend_from_slice(&[S::zero(); 32]); 
                            len = 32; 
                        },
                        "u64" => { 
                            bits.extend_from_slice(&[S::zero(); 64]); 
                            len = 64; 
                        },
                        _ => panic!("unexpected &'static str in IntoField::collect_bits()"),
                    };
                    for i in 0..len {
                        bits[i] = n % NumCast::from(2).expect("NumCast::from(2) in IntoField::collect_bits()");
                        n = n >> 1;
                    };
                    bits
                }).collect::<Vec<_>>();
            Some(
                x.into_iter().map(|y| {
                    y.into_iter().map(|n| {
                        let z = U::from(n);
                        z
                    }).collect::<Vec<U>>()
                }).concat()
            )
        }
    }
}

#[test]
fn test_collect_nums() {
    use zksnark::groth16::fr::FrLocal;
    use serde_json::to_string;

    // expected results are that x_ == y_. and that on a vec len == 0 the fn returns a None.
    let x_none: Vec<usize> = Vec::new();
    assert!(to_string(&x_none.collect_nums::<FrLocal>()).unwrap().contains("null")); 

    let x_8: Vec<usize> = vec![10, 13]; 
    let y_8: Vec<FrLocal> = vec![
        FrLocal::from(10), FrLocal::from(13)
    ];
    match x_8.collect_nums::<FrLocal>() {
        Some(val) => {
            match val == y_8 {
                true => {},
                false => panic!("IntoField::x_8.collect_nums(): left != right"),
            }
        },
        None => panic!("IntoField::x_8.collect_nums() returned a None value"),
    }

    let x_16: Vec<usize> = vec![100, 120]; 
    let y_16: Vec<FrLocal> = vec![
        FrLocal::from(100), FrLocal::from(120)
    ];
    match x_16.collect_nums::<FrLocal>() {
        Some(val) => {
            match val == y_16 {
                true => {},
                false => panic!("IntoField::x_16.collect_nums(): left != right"),
            }
        },
        None => panic!("IntoField::x_16.collect_nums() returned a None value"),
    }

    let x_32: Vec<usize> = vec![1301, 1190]; 
    let y_32: Vec<FrLocal> = vec![
        FrLocal::from(1301), FrLocal::from(1190)
    ];
    match x_32.collect_nums::<FrLocal>() {
        Some(val) => {
            match val == y_32 {
                true => {},
                false => panic!("IntoField::x_32.collect_nums(): left != right"),
            }
        },
        None => panic!("IntoField::x_32.collect_nums() returned a None value"),
    }
}

#[test]
fn test_collect_bits() {
    use zksnark::groth16::fr::FrLocal;
    use serde_json::to_string;

    // expected results are that x_ == y_. and that on a vec len == 0 the fn returns a None.
    let x_none: Vec<usize> = Vec::new();
    assert!(to_string(&x_none.collect_bits::<FrLocal>("")).unwrap().contains("null")); 
    
    let x_8: Vec<usize> = vec![15];
    let y_8: Vec<FrLocal> = vec![
        FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0)
    ];
    
    assert_eq!(y_8.len(), 8);
    match x_8.collect_bits::<FrLocal>("u8") {
        Some(val) => {
            match val == y_8 {
                true => {},
                false => panic!("IntoField::x_8.collect_bits: left != right"),
            };
        },
        None => panic!("IntoField::x_8.collect_bits() returned a None value")
    }

    let x_16: Vec<usize> = vec![1001];
    let y_16: Vec<FrLocal> = vec![
        FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(1), FrLocal::from(0), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_16.len(), 16);
    match x_16.collect_bits::<FrLocal>("u16") {
        Some(val) => {
            match val == y_16 {
                true => {},
                false => panic!("IntoField::x_16.collect_bits: left != right"),
            };
        },
        None => panic!("IntoField:: x_16.collect_bits() returned a None value")
    }

    let x_32: Vec<usize> = vec![30]; 
    let y_32: Vec<FrLocal> = vec![
        FrLocal::from(0), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_32.len(), 32);
    match x_32.collect_bits::<FrLocal>("u32") {
        Some(val) => {
            match val == y_32 {
                true => {},
                false => panic!("IntoField::x_32.collect_bits: left != right"),
            };
        },
        None => panic!("IntoField:: x_32.collect_bits() returned a None value")
    }

    let x_64: Vec<usize> = vec![32];
    let y_64: Vec<FrLocal> = vec![
        FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(1), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0), FrLocal::from(0),
    ];
    assert_eq!(y_64.len(), 64);
    match x_64.collect_bits::<FrLocal>("u64") {
        Some(val) => {
            match val == y_64 {
                true => {},
                false => panic!("IntoField::x_64.collect_bits: left != right"),
            };
        },
        None => panic!("IntoField:: x_64.collect_bits() returned a None value")
    }
}
use itertools::Itertools;
use zksnark::field::Field;

// trait for parsing Vec::<PrimInt> into a vec of field values as either bits or usize.
pub trait IntoField {
    fn collect_nums<T>(self) -> Option<Vec<T>>
    where
        T: Field + From<usize>;
    fn collect_bits<U>(self, t: &String) -> Option<Vec<U>>
    where
        U: Field + From<usize>;
}

// S: PrimInt lets the impl function across u8 -> u64 generically.
// matching the length of the vec and returning a None is currently used to process empty values in the Knowledge struct.
// 'static str solution in fn collect_bits() is required for determining whether a number should be converted to 64, 32 etc bits.
// the string solution is relevant as it enables us to derive bits of a type not necessarily connected to the provided value (within some bounds) e.g. 8 bits for a u32 number.
// TODO: impl guards for asserting a u64 value as 8 bits etc.
impl IntoField for Vec<usize> {
    fn collect_nums<T>(self) -> Option<Vec<T>>
    where
        T: Field + From<usize>
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
    fn collect_bits<U>(self, t: &String) -> Option<Vec<U>> 
    where
        U: Field + From<usize>
    {
        if self.len() == 0 {
            None
        } else {
            let x = self.into_iter()
                .map(|mut n| {
                    let mut bits: Vec<usize> = Vec::new(); 
                    let len: usize;
                    match t.as_ref() {
                        "u8" => { 
                            bits.extend_from_slice(&[0; 8]); 
                            len = 8; 
                        },
                        "u16" => { 
                            bits.extend_from_slice(&[0; 16]); 
                            len = 16; 
                        },
                        "u32" => { 
                            bits.extend_from_slice(&[0; 32]); 
                            len = 32; 
                        },
                        "u64" => { 
                            bits.extend_from_slice(&[0; 64]); 
                            len = 64; 
                        },
                        _ => panic!("unexpected String in t.as_ref() IntoField::collect_bits()"),
                    };
                    for i in 0..len {
                        bits[i] = n % 2;
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
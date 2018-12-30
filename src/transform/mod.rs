use itertools::Itertools;
use num::{PrimInt, NumCast};
use zksnark::field::Field;

pub trait IntoField<S> {
    fn collect_nums<T>(self) -> Option<Vec<T>>
    where
        T: Field + From<S>;
    fn collect_bits<U>(self) -> Option<Vec<U>>
    where
        U: Field + From<S>;
}

// expand somehow for differing [] based on type.
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
                        let x: T = n.into();
                        x
                }).collect::<Vec<T>>()
            )
        }
    }
    fn collect_bits<U>(self) -> Option<Vec<U>> 
    where
        U: Field + From<S>,
    {
        if self.len() == 0 {
            None
        } else {
            let x = self.into_iter()
                .map(|mut n| {
                    let len =  match n {
                        u8 => 8,
                        u16 => 16,
                        u32 => 32,
                        u64 => 64,
                        _ => panic!(),
                    };
                    let mut bits: Vec<S> = vec![];
                    for _ in 0..len {
                        let i: S = n % NumCast::from(2).expect("NumCast::from(2) in IntoField::collect_bits()");
                        bits.push(i);
                        n = n >> 1;
                    };
                    bits
                }).collect::<Vec<_>>();
            Some(
                x.into_iter().map(|y| {
                    y.into_iter().map(|n| {
                        let z: U = n.into();
                        z
                    }).collect::<Vec<U>>()
                }).concat()
            )
        }
    }
}
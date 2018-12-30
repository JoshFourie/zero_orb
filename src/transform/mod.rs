use itertools::Itertools;
use num::{PrimInt, NumCast};
use zksnark::field::Field;

pub trait IntoField<S> {
    fn collect_nums<T>(self) -> Vec<T>
    where
        T: Field + From<S>;
    fn collect_bits<U>(self) -> Vec<U>
    where
        U: Field + From<S>;
}

// expand somehow for differing [] based on type.
impl<S: PrimInt> IntoField<S> for Vec<S> {
    fn collect_nums<T>(self) -> Vec<T>
    where
        T: Field + From<S>,
    {
        self.into_iter()
            .map(|n| {
                let x: T = n.into();
                x
            }).collect::<Vec<T>>()
    }
    fn collect_bits<U>(self) -> Vec<U> 
    where
        U: Field + From<S>,
    {
        let x = self.into_iter()
            .map(|mut n| {
                let mut lim = 0;
                match n {
                    u8 => lim = 8,
                    u16 => lim = 16,
                    u32 => lim = 32,
                    u64 => lim = 64,
                    _ => panic!(),
                };
                let mut bits = [S::zero()];
                for i in 0..lim {
                    bits[i] = n % NumCast::from(2).unwrap();
                    n = n >> 1;
                };
                bits
            }).collect::<Vec<_>>();
        x.into_iter().map(|y| {
            y.into_iter().map(|&n| {
                assert!(n < NumCast::from(30000).unwrap());
                let z: U = n.into();
                z
            }).collect::<Vec<U>>()
        }).concat()
    }
}
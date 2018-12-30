use itertools::Itertools;
use num::{PrimInt, NumCast};
use zksnark::field::Field;

pub trait IntoNumField<F> {
    type Field;
    fn collect_num_field(self) -> Self::Field;
}

impl<U, F> IntoNumField<F> for Vec<U> 
where 
    U: PrimInt, 
    F: Field + From<U>
{
    type Field = Vec<F>;
    fn collect_num_field(self) -> Self::Field {
        self.into_iter()
            .map(|num| {
                let f: F = num.into(); 
                f
            }
        ).collect::<Vec<_>>()
    }
}

pub trait IntoBitsField<F> {
    type Field;
    fn collect_bit_field(self) -> Self::Field;
} 

impl<K, F> IntoBitsField<F> for Vec<K> 
where 
    K: PrimInt,
    F: Field + From<K>
{
        type Field = Vec<F>;
        fn collect_bit_field(self) -> Self::Field {
            let bit_array = self.into_iter()
                .map(|mut num| {
                    let mut bits = [K::zero(); 16];
                    for i in 0..16 {
                        bits[i] = num % NumCast::from(2).unwrap();
                        num = num >> 1;
                    }
                    bits
                }
            ).collect::<Vec<_>>();
            bit_array.into_iter()
                .map(|x| {
                x.iter().map(|&num| {
                    assert!(num < NumCast::from(30000).unwrap());
                    let f: F = num.into();
                    f
                }).collect::<Vec<F>>()        
            }).concat()
        }
    }
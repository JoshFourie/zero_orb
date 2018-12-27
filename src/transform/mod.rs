use itertools::Itertools;
use zksnark::field::z251::Z251;

pub trait IntoNumField {
    type Field;
    fn collect_num_field(self) -> Result<Self::Field, ()>;
}

impl IntoNumField for Vec<u32> {
    type Field = Vec<Z251>;
    fn collect_num_field(self) -> Result<Self::Field, ()> {
        Ok(self.into_iter()
            .map(|num: u32| Z251::from(num as usize))
            .collect::<Vec<_>>()
        )
    }
}

impl IntoNumField for Vec<u8> {
    type Field = Vec<Z251>;
    fn collect_num_field(self) -> Result<Self::Field, ()> {
        Ok(self.into_iter()
            .map(|num: u8| Z251::from(num as usize))
            .collect::<Vec<_>>()
        )
    }
}

impl IntoNumField for Option<Vec<u8>> {
    type Field = Vec<Z251>;
    fn collect_num_field(self) -> Result<Self::Field, ()> {
        match self {
            Some(x) => {
                return Ok(
                    x.into_iter()
                        .map(|num: u8| Z251::from(num as usize))
                        .collect::<Vec<_>>()
                )
            },
            None => Err(()),
        }
    }
}

pub trait IntoBitsField {
    type Field;
    fn collect_bit_field(self) -> Result<Self::Field, ()>;
} 

impl IntoBitsField for Vec<u8> {
    type Field = Vec<Z251>;
    fn collect_bit_field(self) -> Result<Self::Field, ()> {
        let bit_array = self.into_iter().map(|mut num| {
            let mut bits: [u8; 8] = [0; 8];
            for i in 0..8 {
                bits[i] = num % 2;
                num = num >> 1;
            }
            bits
        }).collect::<Vec<_>>();
        Ok(
            bit_array.into_iter()
                .map(|a| {
                a.iter().map(|&n| {
                    assert!(n < 251);
                    Z251 { inner: (n) as u8 }
                }).collect::<Vec<_>>()        
            }).concat()
        )
    }
}

impl IntoBitsField for Option<Vec<u8>> {
    type Field = Vec<Z251>;
    fn collect_bit_field(self) -> Result<Self::Field, ()> {
        match self {
            Some(x) => {
                let bit_array = x.into_iter()
                    .map(|mut num| {
                        let mut bits: [u8; 8] = [0; 8];
                        for i in 0..8 {
                            bits[i] = num % 2;
                            num = num >> 1;
                        }
                        bits
                    }).collect::<Vec<_>>();
                return Ok(
                    bit_array.into_iter()
                        .map(|a| {
                        a.iter().map(|&n| {
                            assert!(n < 251);
                            Z251 { inner: (n) as u8 }
                        }).collect::<Vec<_>>()        
                    }).concat()
                )
            },
            None => Err(())
        }
    }
}

impl IntoBitsField for Vec<u32> {
        type Field = Vec<Z251>;
        fn collect_bit_field(self) -> Result<Self::Field, ()> {
            let bit_array = self.into_iter().map(|mut num| {
                let mut bits: [u32; 32] = [0; 32];
                for i in 0..32 {
                    bits[i] = num % 2;
                    num = num >> 1;
                }
                bits
            }).collect::<Vec<_>>();
            Ok(
                bit_array.into_iter()
                    .map(|a| {
                    a.iter().map(|&n| {
                        assert!(n < 251);
                        Z251 { inner: (n) as u8 }
                    }).collect::<Vec<_>>()        
                }).concat()
            )
        }
    }
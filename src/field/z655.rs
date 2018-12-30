use super::*;
use serde_derive::{Serialize, Deserialize};

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Z655 {
    pub inner: u16,
}

impl Add for Z655 {
    type Output = Z655;
    fn add(self, rhs: Z655) -> Self::Output {
        let sum: u32 = self.inner as u32 + rhs.inner as u32;
        Z655 {
            inner: (sum % 65535) as u16,
        }
    }
}

impl Neg for Z655 {
    type Output = Z655;
    fn neg(self) -> Self::Output {
        Z655 {
            inner: 65535 - self.inner,
        }
    }
}

impl Sub for Z655 {
    type Output = Z655;

    fn sub(self, rhs: Z655) -> Self::Output {
        self + -rhs
    }
}

impl Mul for Z655 {
    type Output = Z655;

    fn mul(self, rhs: Z655) -> Self::Output {
        let product = (self.inner as u32) * (rhs.inner as u32);
        Z655 {
            inner: (product % 65535) as u16,
        }
    }
}

impl Div for Z655 {
    type Output = Z655;
    fn div(self, rhs: Z655) -> Self::Output {
        let (_, mut inv, _) = ext_euc_alg(rhs.inner as isize, 65535);
        while inv < 0 {
            inv += 65535
        }

        self * Z655 { inner: inv as u16 }
    }
}

impl FieldIdentity for Z655 {
    fn zero() -> Self {
        Z655 { inner: 0 }
    }
    fn one() -> Self {
        Z655 { inner: 1 }
    }
}

impl Field for Z655 {
    fn mul_inv(self) -> Self {
        Z655::one().div(self)
    }
}

impl<I: num::PrimInt> From<I> for Z655 {
    fn from(n: I) -> Self {
        use num::NumCast;
        Z655 { inner: NumCast::from(n).expect("NumCast::from(n) for Z655::from<PrimInt>")}
    }
} 

impl Into<usize> for Z655 {
    fn into(self) -> usize {
        self.inner as usize
    }
}

impl FromStr for Z655 {
    type Err = ::std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Z655::from(usize::from_str(s)?))
    }
}

impl Encryptable for Z655 {
    type Output = Z655;

    fn encrypt(self) -> Self::Output {
        let mut ret = Z655::one();
        for _ in 0..self.inner {
            ret = ret * Z655 {inner: 69};
        }
        ret
    }

    fn random() -> Self {
        Z655 {
            inner: rand::random::<u16>() % 65535,
        }
    }
}

impl EncryptProperties for Z655 {
    fn detect_root(&self) -> bool {
        *self == Self::zero()
    }
    fn valid(&self) -> bool {
        true
    }
}

impl Random for Z655 {
    fn random_elem() -> Self {
        let mut r = Z655::random();
        while r == Z655::zero() {
            r = Z655::random()
        }
        r
    }
}

impl EllipticEncryptable for Z655 {
    type G1 = Self;
    type G2 = Self;
    type GT = Self;

    fn encrypt_g1(self) -> Self::G1 {
        self * 69.into()
    }
    fn encrypt_g2(self) -> Self::G2 {
        self * 69.into()
    }
    fn exp_encrypted_g1(self, g1: Self::G1) -> Self::G1 {
        self * g1
    }
    fn exp_encrypted_g2(self, g2: Self::G2) -> Self::G2 {
        self * g2
    }
    fn pairing(g1: Self::G1, g2: Self::G2) -> Self::GT {
        g1 * g2
    }
}

impl Identity for Z655 {
    fn is_identity(&self) -> bool {
        *self == Self::zero()
    }
}

impl Sum for Z655 {
    fn sum<I>(iter: I) -> Self
    where
        I: Iterator<Item = Self>,
    {
        iter.fold(Z655::from(0), |acc, x| acc + x)
    }
}
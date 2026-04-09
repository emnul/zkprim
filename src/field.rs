use std::ops::{Add, Div, Mul, Neg, Sub};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Fp(pub u64);

impl Fp {
    pub const PRIME_MODULUS: u64 = 2u64.pow(61) - 1;

    pub fn new(val: u64) -> Fp {
        return Fp(val % Self::PRIME_MODULUS);
    }
}

impl Add for Fp {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % Self::PRIME_MODULUS)
    }
}

impl Neg for Fp {
    type Output = Self;

    fn neg(self) -> Self::Output {
        // Zero is its own negation
        if self.0 == 0 {
            self
        // Find the additive inverse
        } else {
            Self((Fp::PRIME_MODULUS - self.0) % Fp::PRIME_MODULUS)
        }
    }
}

impl Mul for Fp {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let value = (self.0 * rhs.0) % Fp::PRIME_MODULUS;
        Self(value)
    }
}

impl Sub for Fp {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl Div for Fp {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fp_within_range() {
        let fp = Fp::new(0);
        assert_eq!(fp, Fp(0));

        let fp = Fp::new(Fp::PRIME_MODULUS - 1);
        assert_eq!(fp, Fp(Fp::PRIME_MODULUS - 1));

        let fp = Fp::new(49824);
        assert_eq!(fp, Fp(49824));
    }

    #[test]
    fn create_fp_outside_range() {
        let test_val = Fp::PRIME_MODULUS;
        let fp = Fp::new(test_val);
        assert_eq!(fp, Fp(0));

        let test_val = Fp::PRIME_MODULUS + 42;
        let fp = Fp::new(test_val);
        assert_eq!(fp, Fp(test_val % Fp::PRIME_MODULUS));
    }

    #[test]
    fn add_fp() {
        let test_val1 = 10121;
        let fp1 = Fp::new(test_val1);
        let test_val2 = 414;
        let fp2 = Fp::new(test_val2);

        assert_eq!(fp1 + fp2, Fp((test_val1 + test_val2) % Fp::PRIME_MODULUS));

        let test_val1 = Fp::PRIME_MODULUS - 5;
        let fp1 = Fp::new(test_val1);
        let test_val2 = Fp::PRIME_MODULUS - 5;
        let fp2 = Fp::new(test_val2);

        assert_eq!(fp1 + fp2, Fp((test_val1 + test_val2) % Fp::PRIME_MODULUS));
    }

    #[test]
    fn neg_fp() {
        let a = Fp::new(4);
        assert_eq!(-a, Fp(Fp::PRIME_MODULUS - a.0));

        let a = Fp::new(0);
        assert_eq!(-a, Fp(0));
    }

    fn sub_fp() {}
}

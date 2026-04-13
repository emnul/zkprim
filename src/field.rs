use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, Mul, Neg, Sub},
};

pub struct Fp<M: Modulus = MersennePrime>(pub u64, pub PhantomData<M>);

impl<M: Modulus> Fp<M> {
    pub const PRIME_MODULUS: u64 = M::PRIME_MODULUS;

    pub fn new(val: u64) -> Fp<M> {
        return Fp(val % M::PRIME_MODULUS, PhantomData);
    }

    pub fn batch_multiply<F: FieldElement>(elements: &[F], scalar: &F) -> Vec<F> {
        elements
            .iter()
            .map(|e| e.clone() * scalar.clone())
            .collect()
    }
}

impl<M: Modulus> Clone for Fp<M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<M: Modulus> Copy for Fp<M> {}

impl<M: Modulus> PartialEq for Fp<M> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<M: Modulus> Debug for Fp<M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Fp")
            .field(&self.0)
            .field(&format_args!("_"))
            .finish()
    }
}

impl<M: Modulus> Add for Fp<M> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self((self.0 + rhs.0) % M::PRIME_MODULUS, PhantomData)
    }
}

impl<M: Modulus> Neg for Fp<M> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        // Zero is its own negation
        if self.0 == 0 {
            self
        // Find the additive inverse
        } else {
            Self((M::PRIME_MODULUS - self.0) % M::PRIME_MODULUS, PhantomData)
        }
    }
}

impl<M: Modulus> Mul for Fp<M> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        // TODO: This naive implementation is inefficient, but it avoids an overflow problem
        // In the future, we can improve this using Montgomery multiplication
        let value = (self.0 as u128 * rhs.0 as u128) % M::PRIME_MODULUS as u128;
        Self(value as u64, PhantomData)
    }
}

impl<M: Modulus> Sub for Fp<M> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

pub trait FieldElement: Sized + Clone + Mul<Output = Self> {
    fn zero() -> Self;
    fn one() -> Self;
    fn inv(&self) -> Option<Self>;
    fn pow(&self, exp: u64) -> Self;
}

impl<M: Modulus> FieldElement for Fp<M> {
    // Use fast exponentiation algorithm https://math-sites.uncg.edu/sites/pauli/112/HTML/secfastexp.html
    fn pow(&self, mut exp: u64) -> Self {
        let mut res = Fp::one();
        let mut base = *self;

        while exp != 0 {
            if exp & 1 == 1 {
                res = res * base;
            }
            exp >>= 1;
            base = base * base;
        }

        res
    }

    fn one() -> Self {
        Fp(1, PhantomData)
    }

    fn zero() -> Self {
        Fp(0, PhantomData)
    }

    // Returns the multiplicative inverse of Fp via an extension of
    // Fermat's Little Theorem
    fn inv(&self) -> Option<Self> {
        if *self == Self::zero() {
            return None;
        }

        // a^p-1 = 1 mod p
        // a^p-1 * a^-1 = a^-1 mod p
        // a^p-2 = a^-1 mod p
        Some(self.pow(M::PRIME_MODULUS - 2))
    }
}

pub trait Modulus {
    const PRIME_MODULUS: u64;
}

pub struct MersennePrime;

impl Modulus for MersennePrime {
    const PRIME_MODULUS: u64 = 2u64.pow(61) - 1;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_fp_within_range() {
        let fp: Fp = Fp::new(0);
        assert_eq!(fp, Fp::new(0));

        let fp: Fp = Fp::new(MersennePrime::PRIME_MODULUS - 1);
        assert_eq!(fp, Fp::new(MersennePrime::PRIME_MODULUS - 1));

        let fp: Fp = Fp::new(49824);
        assert_eq!(fp, Fp::new(49824));
    }

    #[test]
    fn create_fp_outside_range() {
        let test_val = MersennePrime::PRIME_MODULUS;
        let fp: Fp = Fp::new(test_val);
        assert_eq!(fp, Fp::new(0));

        let test_val = MersennePrime::PRIME_MODULUS + 42;
        let fp: Fp = Fp::new(test_val);
        assert_eq!(fp, Fp::new(test_val % MersennePrime::PRIME_MODULUS));
    }

    #[test]
    fn add_fp() {
        let test_val1 = 10121;
        let fp1: Fp = Fp::new(test_val1);
        let test_val2 = 414;
        let fp2 = Fp::new(test_val2);

        assert_eq!(
            fp1 + fp2,
            Fp::new((test_val1 + test_val2) % MersennePrime::PRIME_MODULUS)
        );

        let test_val1 = MersennePrime::PRIME_MODULUS - 5;
        let fp1: Fp = Fp::new(test_val1);
        let test_val2 = MersennePrime::PRIME_MODULUS - 5;
        let fp2 = Fp::new(test_val2);

        assert_eq!(
            fp1 + fp2,
            Fp::new((test_val1 + test_val2) % MersennePrime::PRIME_MODULUS)
        );
    }

    #[test]
    fn sub_fp() {
        let test_val1 = 10121;
        let fp1: Fp = Fp::new(test_val1);
        let test_val2 = 414;
        let fp2 = Fp::new(test_val2);

        assert_eq!(
            fp1 - fp2,
            Fp::new((test_val1 - test_val2) % MersennePrime::PRIME_MODULUS)
        );

        let test_val1 = MersennePrime::PRIME_MODULUS - 5;
        let fp1: Fp = Fp::new(test_val1);
        let test_val2 = MersennePrime::PRIME_MODULUS - 5;
        let fp2 = Fp::new(test_val2);

        assert_eq!(fp1 - fp2, Fp::new(0));

        let test_val1 = 10;
        let fp1: Fp = Fp::new(test_val1);
        let test_val2 = 20;
        let fp2 = Fp::new(test_val2);

        assert!(test_val2 > test_val1);

        assert_eq!(
            fp1 - fp2,
            Fp::new(fp1.0 + (MersennePrime::PRIME_MODULUS - fp2.0) % MersennePrime::PRIME_MODULUS)
        );
    }

    #[test]
    fn neg_fp() {
        let a: Fp = Fp::new(4);
        assert_eq!(-a, Fp::new(MersennePrime::PRIME_MODULUS - a.0));

        let a: Fp = Fp::new(0);
        assert_eq!(-a, Fp::new(0));
    }

    #[test]
    fn mul_fp() {
        let test_val1 = 10121;
        let fp1: Fp = Fp::new(test_val1);
        let test_val2 = 414;
        let fp2 = Fp::new(test_val2);

        assert_eq!(
            fp1 * fp2,
            Fp::new(
                ((test_val1 as u128 * test_val2 as u128) % MersennePrime::PRIME_MODULUS as u128)
                    as u64
            )
        );

        let test_val1 = MersennePrime::PRIME_MODULUS - 5;
        let fp1: Fp = Fp::new(test_val1);
        let test_val2 = MersennePrime::PRIME_MODULUS - 5;
        let fp2 = Fp::new(test_val2);

        assert_eq!(
            fp1 * fp2,
            Fp::new(
                ((test_val1 as u128 * test_val2 as u128) % MersennePrime::PRIME_MODULUS as u128)
                    as u64
            )
        );
    }

    #[test]
    fn pow_fp() {
        let test_val1 = 10;
        let fp1: Fp = Fp::new(test_val1);
        let exp = 10;

        assert_eq!(fp1.pow(exp), Fp::new(test_val1.pow(exp as u32)));

        let test_val1 = 2;
        let fp1: Fp = Fp::new(test_val1);
        let exp = 61;

        assert_eq!(fp1.pow(exp), Fp::new(test_val1.pow(exp as u32)));
        assert_eq!(fp1.pow(exp), Fp::one());

        // Use Fermat's Little Theorem to test pow
        let test_val1 = 3234; // a can be any number in the group
        let exp = MersennePrime::PRIME_MODULUS - 1; // Group order is p - 1
        let fp1: Fp = Fp::new(test_val1);

        assert_eq!(fp1.pow(exp), Fp::one());
    }

    #[test]
    fn inv_fp() {
        // 1/0 is undefined
        assert_eq!(Fp::<MersennePrime>::zero().inv(), None);

        // inverse of the identity element is itself
        assert_eq!(Fp::<MersennePrime>::one().inv(), Some(Fp::one()));

        // Use Fermant's Little Theorem to test inv
        assert_eq!(
            Fp::<MersennePrime>::new(10)
                .pow(MersennePrime::PRIME_MODULUS - 1)
                .inv(),
            Some(Fp::one())
        );
    }
}

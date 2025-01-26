use rand_core::{RngCore, OsRng};
use subtle::ConstantTimeEq;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};


/// An element of the given Field.
#[derive(Debug, Clone, Copy)]
pub struct FieldElement<const MODULUS: u64> {
    value: u64,
}

impl<const MODULUS: u64> FieldElement<MODULUS> {
    pub fn new(value: u64) -> Self {
        FieldElement {
            value: value % MODULUS,
        }
    }

    pub fn zero() -> Self {
        FieldElement { value: 0 }
    }

    pub fn one() -> Self {
        FieldElement { value: 1 }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn random() -> Self {
        let mut rng = OsRng;
        let value = rng.next_u64() % MODULUS;
        FieldElement::new(value)
    }

    /// Modular exponentiation using a constant-time algorithm.
    pub fn pow(&self, exp: u64) -> Self {
        let mut result = 1u64;
        let mut base = self.value;
        let mut e = exp;

        while e > 0 {
            if e & 1 == 1 {
                result = (result * base) % MODULUS;
            }
            base = (base * base) % MODULUS;
            e >>= 1;
        }
        FieldElement { value: result }
    }

    /// `a^(p-2) % p`.
    pub fn inverse(&self) -> Self {
        assert!(MODULUS > 2, "Modulus must be > 2 for inverse calculation");
        self.pow(MODULUS - 2)
    }

    pub fn to_bytes(&self) -> [u8; 8] {
        self.value.to_be_bytes() //big endian
    }
}

impl<const MODULUS: u64> PartialEq for FieldElement<MODULUS> {
    fn eq(&self, other: &Self) -> bool {
        self.value.ct_eq(&other.value).unwrap_u8() == 1
    }
}

impl<const MODULUS: u64> Eq for FieldElement<MODULUS> {}

impl<const MODULUS: u64> Add for FieldElement<MODULUS> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        FieldElement::new(self.value + rhs.value)
    }
}

impl<const MODULUS: u64> AddAssign for FieldElement<MODULUS> {
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) % MODULUS;
    }
}

impl<const MODULUS: u64> Sub for FieldElement<MODULUS> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        let diff = MODULUS + self.value - rhs.value;
        FieldElement::new(diff % MODULUS)
    }
}

impl<const MODULUS: u64> SubAssign for FieldElement<MODULUS> {
    fn sub_assign(&mut self, rhs: Self) {
        let diff = MODULUS + self.value - rhs.value;
        self.value = diff % MODULUS;
    }
}

impl<const MODULUS: u64> Mul for FieldElement<MODULUS> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        FieldElement::new((self.value as u128 * rhs.value as u128 % MODULUS as u128) as u64)
    }
}

impl<const MODULUS: u64> MulAssign for FieldElement<MODULUS> {
    fn mul_assign(&mut self, rhs: Self) {
        self.value = (self.value as u128 * rhs.value as u128 % MODULUS as u128) as u64;
    }
}

impl<const MODULUS: u64> Div for FieldElement<MODULUS> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inverse()
    }
}

impl<const MODULUS: u64> DivAssign for FieldElement<MODULUS> {
    fn div_assign(&mut self, rhs: Self) {
        *self = *self * rhs.inverse();
    }
}

impl<const MODULUS: u64> Neg for FieldElement<MODULUS> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        FieldElement::new(MODULUS - self.value)
    }
}
#[cfg(test)]
mod test_field_operations {
    use super::*;

    #[test]
    fn test_field_add() {
        let a = FieldElement::<7>::new(1);
        let b = FieldElement::<7>::new(2);
        let c = a + b;
        assert_eq!(c.value(), 3);
    }

    #[test]
    fn test_field_sub() {
        let a = FieldElement::<7>::new(1);
        let b = FieldElement::<7>::new(2);
        let c = a - b;
        assert_eq!(c.value(), 6);
    }

    #[test]
    fn test_field_mul() {
        let a = FieldElement::<7>::new(3);
        let b = FieldElement::<7>::new(4);
        let c = a * b;
        assert_eq!(c.value(), 5);
    }

    #[test]
    fn test_field_div() {
        let a = FieldElement::<7>::new(1);
        let b = FieldElement::<7>::new(3);
        let c = a / b;
        assert_eq!(c.value(), 5);
    }

    #[test]
    fn test_field_inverse() {
        let a = FieldElement::<7>::new(3);
        let b = a.inverse();
        assert_eq!(b.value(), 5);
    }

    #[test]
    fn test_field_pow() {
        let a = FieldElement::<7>::new(3);
        let b = a.pow(3);
        assert_eq!(b.value(), 6);
    }

    #[test]
    fn test_zero_and_one() {
        let zero = FieldElement::<7>::zero();
        let one = FieldElement::<7>::one();
        assert_eq!(zero.value(), 0);
        assert_eq!(one.value(), 1);
    }

    #[test]
    fn test_negation() {
        let a = FieldElement::<7>::new(3);
        let b = -a;
        assert_eq!(b.value(), 4);
    }

    #[test]
    fn test_random_generation() {
        for _ in 0..100 {
            let random_element = FieldElement::<7>::random();
            assert!(random_element.value() < 7);
        }
    }

    #[test]
    fn test_modular_wraparound() {
        let a = FieldElement::<7>::new(10);
        let b = FieldElement::<7>::new(12);
        let c = a + b;
        assert_eq!(c.value(), 1);
    }

    #[test]
    fn test_equality() {
        let a = FieldElement::<7>::new(3);
        let b = FieldElement::<7>::new(10); // Both 3 % 7 and 10 % 7 reduce to 3
        assert!(a == b);
    }

    #[test]
    fn test_field_add_assign() {
        let mut a = FieldElement::<7>::new(3);
        let b = FieldElement::<7>::new(5);
        a += b;
        assert_eq!(a.value(), 1);
    }

    #[test]
    fn test_field_sub_assign() {
        let mut a = FieldElement::<7>::new(3);
        let b = FieldElement::<7>::new(5);
        a -= b;
        assert_eq!(a.value(), 5);
    }

    #[test]
    fn test_field_mul_assign() {
        let mut a = FieldElement::<7>::new(3);
        let b = FieldElement::<7>::new(5);
        a *= b;
        assert_eq!(a.value(), 1);
    }

    #[test]
    fn test_field_div_assign() {
        let mut a = FieldElement::<7>::new(3);
        let b = FieldElement::<7>::new(5);
        a /= b;
        assert_eq!(a.value(), 2);
    }


    #[test]
    fn test_pow_zero() {
        let a = FieldElement::<7>::new(3);
        let result = a.pow(0);
        assert_eq!(result.value(), 1);
    }

    #[test]
    fn test_pow_one() {
        let a = FieldElement::<7>::new(3);
        let result = a.pow(1);
        assert_eq!(result.value(), 3);
    }

    #[test]
    fn test_inverse_multiplication() {
        let a = FieldElement::<7>::new(3);
        let inv = a.inverse();
        assert_eq!((a * inv).value(), 1);
    }
}

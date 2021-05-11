use crate::forward_ref_binop;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use bigint::U256;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct FieldElement {
    pub num: U256,
    pub prime: U256,
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl FieldElement {
    pub fn new(num: U256, prime: U256) -> Self {
        if num >= prime {
            panic!("Num {} not in field range 0 to {}", num, prime);
        }

        FieldElement { num, prime }
    }

    pub fn pow(&self, num: U256) -> Self {
        let mod_pow = |mut base: U256, mut exp: U256, modulus: U256| {
            if modulus == 1.into() {
                return 0;
            }
            let mut result: U256 = 1.into();
            base = base % modulus; while exp > 0.into() {
                if exp % 2.into() == 1.into() {
                    result = result * base % modulus;
                }
                exp = exp >> 1;
                base = base * base % modulus
            }
            result
        };
        let n = num % (self.prime - 1.into());
        let new_num = mod_pow(self.num, n, self.prime).into() % self.prime;
        FieldElement::new(new_num, self.prime)
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    fn add(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        let new_num = (self.num + other.num) % self.prime;
        FieldElement::new(new_num, self.prime)
    }
}
forward_ref_binop! { impl Add, add for FieldElement }

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        let new_other = FieldElement::new((-1.into() * other.num).rem_euclid(self.prime), self.prime);
        self + new_other
    }
}
forward_ref_binop! { impl Sub, sub for FieldElement }

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        FieldElement::new((self.num * other.num) % self.prime, self.prime)
    }
}
forward_ref_binop! { impl Mul, mul for FieldElement }

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, other: FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        other.pow(self.prime - 2.into()) * self
    }
}
forward_ref_binop! { impl Div, div for FieldElement }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test() {
        let a = FieldElement::new(7, 13);
        assert_eq!(a, a);
    }

    #[test]
    fn equality_test2() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(6, 13);
        assert_ne!(a, b);
    }

    #[test]
    fn add_test() {
        let a = FieldElement::new(7, 13);
        let b = FieldElement::new(12, 13);
        let c = FieldElement::new(6, 13);
        assert_eq!(&a + &b, c);
    }

    #[test]
    fn sub_test() {
        let a = FieldElement::new(6, 19);
        let b = FieldElement::new(13, 19);
        let c = FieldElement::new(12, 19);
        assert_eq!(&a - &b, c);
    }

    #[test]
    fn mul_test() {
        let a = FieldElement::new(8, 19);
        let b = FieldElement::new(17, 19);
        let c = FieldElement::new(3, 19);
        assert_eq!(&a * &b, c);
    }

    #[test]
    fn pow_test() {
        let a = FieldElement::new(3, 13);
        let b = FieldElement::new(1, 13);
        assert_eq!(a.pow(3), b);
    }

    #[test]
    fn div_test() {
        let a = FieldElement::new(2, 19);
        let b = FieldElement::new(7, 19);
        let c = FieldElement::new(3, 19);
        assert_eq!(&a / &b, c);
    }

    #[test]
    fn pow_test2() {
        let a = FieldElement::new(17, 31);
        let b = FieldElement::new(29, 31);
        assert_eq!(a.pow(-3), b);
    }
}

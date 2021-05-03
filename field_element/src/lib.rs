use std::fmt;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, PartialEq)]
struct FieldElement {
    num: i32,
    prime: i32,
}

impl fmt::Display for FieldElement {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "FieldElement_{}({})", self.prime, self.num)
    }
}

impl FieldElement {
    pub fn new(num: i32, prime: i32) -> Self {
        if num >= prime || num < 0 {
            panic!("Num {} not in field range 0 to {}", num, prime);
        }

        FieldElement { num, prime }
    }

    pub fn pow(&self, num: i32) -> Self {
        let mod_pow = |mut base: i32, mut exp: i32, modulus: i32| {
            if modulus == 1 {
                return 0;
            }
            let mut result = 1;
            base = base % modulus;
            while exp > 0 {
                if exp % 2 == 1 {
                    result = result * base % modulus;
                }
                exp = exp >> 1;
                base = base * base % modulus
            }
            result
        };
        let n = num.rem_euclid(self.prime - 1);
        let new_num = mod_pow(self.num, n, self.prime).rem_euclid(self.prime);
        FieldElement::new(new_num, self.prime)
    }
}

impl<'a, 'b> Add<&'b FieldElement> for &'a FieldElement {
    type Output = FieldElement;

    fn add(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        let new_num = (self.num + other.num).rem_euclid(self.prime);
        FieldElement::new(new_num, self.prime)
    }
}

impl<'a, 'b> Sub<&'b FieldElement> for &'a FieldElement  {
    type Output = FieldElement;

    fn sub(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        let new_other = FieldElement::new((-1 * other.num).rem_euclid(self.prime), self.prime);
        self + &new_other
    }
}

impl<'a, 'b> Mul<&'b FieldElement> for &'a FieldElement  {
    type Output = FieldElement;

    fn mul(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        FieldElement::new((self.num * other.num).rem_euclid(self.prime), self.prime)
    }
}

impl<'a, 'b> Div<&'b FieldElement> for &'a FieldElement  {
    type Output = FieldElement;

    fn div(self, other: &'b FieldElement) -> FieldElement {
        if self.prime != other.prime {
            panic!("Cannot add two numbers in different Fields");
        }

        &other.pow(self.prime - 2) * self
    }
}

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

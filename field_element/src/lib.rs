use std::fmt;
use std::ops::{Add, Mul, Sub};

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
}

impl Add for FieldElement {
    type Output = Result<Self, &'static str>;

    fn add(self, other: Self) -> Result<Self, &'static str> {
        if self.prime != other.prime {
            return Err("Cannot add two numbers in different Fields");
        }

        let new_num = (self.num + other.num).rem_euclid(self.prime);
        Ok(FieldElement::new(new_num, self.prime))
    }
}

impl Sub for FieldElement {
    type Output = Result<Self, &'static str>;

    fn sub(self, other: Self) -> Result<Self, &'static str> {
        if self.prime != other.prime {
            return Err("Cannot add two numbers in different Fields");
        }

        let new_other = FieldElement::new((-1 * other.num).rem_euclid(self.prime), self.prime);
        self + new_other
    }
}

impl Mul for FieldElement {
    type Output = Result<Self, &'static str>;

    fn mul(self, other: Self) -> Result<Self, &'static str> {
        if self.prime != other.prime {
            return Err("Cannot add two numbers in different Fields");
        }

        Ok(FieldElement::new(
            (self.num * other.num).rem_euclid(self.prime),
            self.prime,
        ))
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
        assert_eq!(a + b, Ok(c));
    }

    #[test]
    fn add_test2() {
        let a = FieldElement::new(6, 19);
        let b = FieldElement::new(12, 13);
        assert_eq!(a + b, Err("Cannot add two numbers in different Fields"));
    }

    #[test]
    fn sub_test() {
        let a = FieldElement::new(6, 19);
        let b = FieldElement::new(13, 19);
        let c = FieldElement::new(12, 19);
        assert_eq!(a - b, Ok(c));
    }

    #[test]
    fn mul_test() {
        let a = FieldElement::new(8, 19);
        let b = FieldElement::new(17, 19);
        let c = FieldElement::new(3, 19);
        assert_eq!(a * b, Ok(c));
    }
}

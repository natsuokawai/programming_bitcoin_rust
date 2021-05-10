use crate::field_element::FieldElement;
use crate::point::Point;
use crate::point::Coordinate;
use lazy_static::lazy_static;
use std::ops::{Add, Mul};

#[derive(Clone, Copy, Debug, PartialEq)]
struct S256Field {
    field: FieldElement,
}

lazy_static! {
    static ref P: i64 = 2i64.pow(256) - 2i64.pow(32) - 977;
    static ref N: i64 = "0xfffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141".parse().expect("hardcoded value should parse without errors");
    static ref GX: i64 = "0x79be667ef9dcbbac55a06295ce870b07029bfcdb2dce28d959f2815b16f81798".parse().expect("hardcoded value should parse without errors");
    static ref GY: i64 = "0x483ada7726a3c4655da4fbfc0e1108a8fd17b448a68554199c47d08ffb10d4b8".parse().expect("hardcoded value should parse without errors");
}

impl S256Field {
    pub fn new(num: i64) -> Self {
        S256Field {
            field: FieldElement::new(num, *P),
        }
    }
}

const A: i64 = 0;
const B: i64 = 7;

#[derive(Clone, Copy, Debug, PartialEq)]
struct S256Point {
    point: Point,
}

impl S256Point {
    fn new(x: i64, y: i64) -> Self {
        let a = S256Field::new(A);
        let b = S256Field::new(B);
        S256Point {
            point: Point {
                x: Coordinate::Num(S256Field::new(x).field),
                y: Coordinate::Num(S256Field::new(y).field),
                a: a.field,
                b: b.field,
            }
        }
    }

    fn inf() -> Self {
        let a = S256Field::new(A);
        let b = S256Field::new(B);
        S256Point {
            point: Point {
                x: Coordinate::Inf,
                y: Coordinate::Inf,
                a: a.field,
                b: b.field,
            }
        }
    }
}

impl Add for S256Point {
    type Output = S256Point;

    fn add(self, other: S256Point) -> S256Point {
        S256Point { point: self.point + other.point }
    }
}

impl Mul<S256Point> for i64 {
    type Output = S256Point;

    fn mul(self, other: S256Point) -> S256Point {
        S256Point { point: (self % *N) * other.point }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mul_test() {
        let p1 = S256Point::new(*GX, *GY);
        let p2 = S256Point::inf();
        assert_eq!(*N * p1, p2);
    }
}

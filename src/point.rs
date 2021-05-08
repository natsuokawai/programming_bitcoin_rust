use crate::field_element::FieldElement;
use crate::forward_ref_binop;
use std::fmt;
use std::ops::{Add, Mul};

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Coordinate {
    Num(FieldElement),
    Inf,
}

impl Coordinate {
    fn num(self) -> FieldElement {
        match self {
            Coordinate::Num(x) => x,
            Coordinate::Inf => panic!("not a number"),
        }
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Coordinate::Num(x) => x.to_string(),
                _ => String::from("Inf"),
            }
        )
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Point {
    pub a: FieldElement,
    pub b: FieldElement,
    pub x: Coordinate,
    pub y: Coordinate,
}

impl Point {
    fn new(x: Coordinate, y: Coordinate, a: FieldElement, b: FieldElement) -> Self {
        let result = Point { a, b, x, y };

        if !result.is_on_curve() {
            panic!("({}, {}) is not on the curve.", result.x, result.y);
        }

        result
    }

    fn is_on_curve(&self) -> bool {
        match (&self.x, &self.y) {
            (Coordinate::Inf, Coordinate::Inf) => true,
            (Coordinate::Num(x), Coordinate::Num(y)) => y.pow(2) == x.pow(3) + self.a * x + self.b,
            (_, _) => false,
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        match (&self.x, &other.x) {
            // Inf is the unit source in addition
            (Coordinate::Inf, _) => other,
            (_, Coordinate::Inf) => self,
            (_, _) => {
                let x1 = &self.x.num();
                let y1 = &self.y.num();
                let x2 = &other.x.num();
                let y2 = &other.y.num();
                let p = x1.prime;

                // Intersection of a line passing through x1 and x2 with an elliptic curve
                if x1 != x2 {
                    let s = (y2 - y1) / (x2 - x1);
                    let x3 = &s.pow(2) - x1 - x2;
                    let y3 = &s * (x1 - &x3) - y1;
                    return Point::new(Coordinate::Num(x3), Coordinate::Num(y3), self.a, self.b);
                }

                // When it is a tangent line
                if y1 == y2 && y1 != &FieldElement::new(0, p) {
                    let s = (FieldElement::new(3, p) * x1.pow(2) + &self.a)
                        / (FieldElement::new(2, p) * y1);
                    let x3 = &s.pow(2) - FieldElement::new(2, p) * x1;
                    let y3 = &s * (x1 - &x3) - y1;
                    return Point::new(Coordinate::Num(x3), Coordinate::Num(y3), self.a, self.b);
                }

                // When the slope is zero (vertical)
                Point::new(Coordinate::Inf, Coordinate::Inf, self.a, self.b)
            }
        }
    }
}
forward_ref_binop! { impl Add, add for Point }

impl Mul<Point> for i64 {
    type Output = Point;

    fn mul(self, other: Point) -> Point {
        let mut result = Point::new(Coordinate::Inf, Coordinate::Inf, other.a, other.b);
        for _ in 0..self {
            result = result + &other;
        }
        result
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "x: {}, y: {} (y^2 = x^3 + {}x + {})",
            self.x, self.y, self.a, self.b
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equality_test() {
        let prime = 223;
        let x = Coordinate::Num(FieldElement::new(192, prime));
        let y = Coordinate::Num(FieldElement::new(105, prime));
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let p = Point::new(x, y, a, b);
        assert_eq!(p, p);
    }

    #[test]
    fn add_test_1() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let x1 = Coordinate::Num(FieldElement::new(170, prime));
        let y1 = Coordinate::Num(FieldElement::new(142, prime));
        let p1 = Point::new(x1, y1, a, b);
        let x2 = Coordinate::Num(FieldElement::new(60, prime));
        let y2 = Coordinate::Num(FieldElement::new(139, prime));
        let p2 = Point::new(x2, y2, a, b);
        let x3 = Coordinate::Num(FieldElement::new(220, prime));
        let y3 = Coordinate::Num(FieldElement::new(181, prime));
        let p3 = Point::new(x3, y3, a, b);
        assert_eq!(p1 + p2, p3);
    }

    #[test]
    fn add_test_2() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let x1 = Coordinate::Num(FieldElement::new(192, prime));
        let y1 = Coordinate::Num(FieldElement::new(105, prime));
        let p1 = Point::new(x1, y1, a, b);
        let x2 = Coordinate::Num(FieldElement::new(49, prime));
        let y2 = Coordinate::Num(FieldElement::new(71, prime));
        let p2 = Point::new(x2, y2, a, b);
        assert_eq!(&p1 + p1, p2);
    }

    #[test]
    fn scalar_multiplication_test() {
        let prime = 223;
        let a = FieldElement::new(0, prime);
        let b = FieldElement::new(7, prime);
        let x1 = Coordinate::Num(FieldElement::new(15, prime));
        let y1 = Coordinate::Num(FieldElement::new(86, prime));
        let p1 = Point::new(x1, y1, a, b);
        let x2 = Coordinate::Inf;
        let y2 = Coordinate::Inf;
        let p2 = Point::new(x2, y2, a, b);
        assert_eq!(7 * p1, p2);
    }
}

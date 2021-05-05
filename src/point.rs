use std::fmt;
use std::ops::Add;
use crate::field_element::FieldElement;

#[derive(PartialEq, Debug)]
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

#[derive(PartialEq, Debug)]
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
            (Coordinate::Num(x), Coordinate::Num(y)) => y.pow(2) == &(&x.pow(3) + &(&self.a * &x)) + &self.b,
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
                    let s = &(y2 - y1) / &(x2 - x1);
                    let x3 = &(&s.pow(2) - x1) - x2;
                    let y3 = &(&s * &(x1 - &x3)) - y1;
                    return Point::new(Coordinate::Num(x3), Coordinate::Num(y3), self.a, self.b);
                }

                // When it is a tangent line
                if y1 == y2 && y1 != &FieldElement::new(0, p) {
                    let s = &(&(&FieldElement::new(3, p) * &x1.pow(2)) + &self.a) / &(&FieldElement::new(2, p) * y1);
                    let x3 = &(&s.pow(2) - &FieldElement::new(2, p)) * x1;
                    let y3 = &(&s * &(x1 - &x3)) - y1;
                    return Point::new(Coordinate::Num(x3), Coordinate::Num(y3), self.a, self.b);
                }

                // When the slope is zero (vertical)
                Point::new(Coordinate::Inf, Coordinate::Inf, self.a, self.b)
            }
        }
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
}

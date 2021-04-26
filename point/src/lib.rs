use std::fmt;
use std::ops::Add;

#[derive(PartialEq, Debug)]
enum Coordinate {
    Num(i32),
    Inf,
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
struct Point {
    a: i32,
    b: i32,
    x: Coordinate,
    y: Coordinate,
}

impl Point {
    fn new(x: Coordinate, y: Coordinate, a: i32, b: i32) -> Self {
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
            (Coordinate::Inf, _) => other,
            (_, Coordinate::Inf) => self,
            (_, _) => {
                self // wip
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
        let p = Point::new(Coordinate::Num(18), Coordinate::Num(77), 5, 7);
        assert_eq!(p, p);
    }

    #[test]
    fn add_infinity_point_test() {
        let p = Point::new(Coordinate::Num(-1), Coordinate::Num(-1), 5, 7);
        let inf = Point::new(Coordinate::Inf, Coordinate::Inf, 5, 7);
        let p_ = Point::new(Coordinate::Num(-1), Coordinate::Num(-1), 5, 7);
        assert_eq!(p + inf, p_);
    }
}

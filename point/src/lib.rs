use std::fmt;

#[derive(PartialEq, Debug)]
struct Point {
    a: i32,
    b: i32,
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32, a: i32, b: i32) -> Self {
        let result = Point {
            a, b, x, y
        };

        if !result.is_on_curve() {
            panic!("({}, {}) is not on the curve.", x, y);
        }

        result
    }

    fn is_on_curve(&self) -> bool {
        self.y.pow(2) == self.x.pow(3) + self.a * self.x + self.b
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
        let p = Point::new(18, 77, 5, 7);
        assert_eq!(p, p);
    }
}

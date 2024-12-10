use std::{
    cmp::Ordering,
    fmt::Display,
    ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign},
};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct Point(pub i32, pub i32);

impl Point {
    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn x_usize(&self) -> usize {
        self.0 as usize
    }

    pub fn y_usize(&self) -> usize {
        self.1 as usize
    }

    pub fn diff(&self, rhs: &Point) -> Point {
        Point::from((self.0.abs_diff(rhs.0), self.1.abs_diff(rhs.1)))
    }

    pub fn origin() -> Point {
        Point(0, 0)
    }
}

impl From<(usize, usize)> for Point {
    fn from(value: (usize, usize)) -> Self {
        Self(value.0 as i32, value.1 as i32)
    }
}

impl From<(u32, u32)> for Point {
    fn from(value: (u32, u32)) -> Self {
        Self(value.0 as i32, value.1 as i32)
    }
}

impl From<(i32, i32)> for Point {
    fn from(value: (i32, i32)) -> Self {
        Self(value.0, value.1)
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl SubAssign for Point {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
        self.1 -= rhs.1;
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, rhs: i32) -> Self::Output {
        Self(self.0 * rhs, self.1 * rhs)
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Self(-self.0, -self.1)
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        let lhs = self.0.cmp(&other.0);
        let rhs = self.1.cmp(&other.1);

        match (lhs, rhs) {
            (Ordering::Equal, Ordering::Equal) => Ordering::Equal,
            (Ordering::Equal, Ordering::Greater) => Ordering::Greater,
            (Ordering::Equal, Ordering::Less) => Ordering::Less,
            (Ordering::Greater, Ordering::Equal) => Ordering::Greater,
            (Ordering::Greater, Ordering::Greater) => Ordering::Greater,
            (Ordering::Greater, Ordering::Less) => Ordering::Less,
            (Ordering::Less, Ordering::Equal) => Ordering::Less,
            (Ordering::Less, Ordering::Less) => Ordering::Less,
            (Ordering::Less, Ordering::Greater) => Ordering::Less,
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

#[cfg(test)]
mod tests {
    use crate::models::point::Point;

    #[test]
    fn subtraction_should_work() {
        let point = Point::from((2, 2));

        let expected = Point::from((1, 1));

        assert_eq!(expected, point - Point::from((1, 1,)))
    }

    #[test]
    fn addition_should_work() {
        let point = Point::from((2, 2));

        let expected = Point::from((3, 3));

        assert_eq!(expected, point + Point::from((1, 1,)))
    }

    #[test]
    fn diff_should_work() {
        let a = Point::from((1, 1));
        let b = Point::from((3, 3));

        assert_eq!(a.diff(&b), Point::from((2, 2)));
    }

    #[test]
    fn diff_should_follow_associative_rules() {
        let a = Point::from((1, 1));
        let b = Point::from((3, 3));

        assert_eq!(b.diff(&a), Point::from((2, 2)));
    }

    #[test]
    fn greater_than_should_work() {
        let a = Point::from((0, 0));
        let b = Point::from((1, 1));

        assert!(b > a)
    }

    #[test]
    fn less_than_should_work() {
        let a = Point::from((0, 0));
        let b = Point::from((1, 1));

        assert!(b < a)
    }

    #[test]
    fn comparison_works() {
        let origin = Point::origin();

        assert!(Point::from((-1, 1)) < origin);
        assert!(Point::from((-1, 0)) < origin);
        assert!(Point::from((-1, -1)) < origin);
        assert!(Point::from((0, -1)) < origin);
        assert!(Point::from((1, -1)) < origin);
        assert!(Point::from((1, 0)) > origin);
        assert!(Point::from((1, 1)) > origin);
        assert!(Point::from((0, 1)) > origin);
    }
}

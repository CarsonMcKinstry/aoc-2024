use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, Default)]
pub(crate) struct Position {
    x: usize,
    y: usize,
}

impl Position {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub(crate) fn distance_to(&self, other: &Position) -> Distance {
        let d_x = self.x.abs_diff(other.x);
        let d_y = self.y.abs_diff(other.y);

        Distance::new(d_x, d_y)
    }

    pub(crate) fn add(&self, x: isize, y: isize) -> Position {
        Self {
            x: ((self.x as isize) + x) as usize,
            y: ((self.y as isize) + y) as usize,
        }
    }

    pub(crate) fn x(&self) -> usize {
        self.x
    }

    pub(crate) fn y(&self) -> usize {
        self.y
    }

    pub(crate) fn compare_y(&self, other: &Position) -> std::cmp::Ordering {
        self.y.cmp(&other.y) // Compare based on the y-coordinate
    }

    pub(crate) fn compare_x(&self, other: &Position) -> std::cmp::Ordering {
        self.x.cmp(&other.x) // Compare based on the x-coordinate
    }

    pub(crate) fn north_of(&self, from: &Position) -> bool {
        self.y < from.y && self.x == from.x
    }

    pub(crate) fn south_of(&self, from: &Position) -> bool {
        self.y > from.y && self.x == from.x
    }

    pub(crate) fn east_of(&self, from: &Position) -> bool {
        self.x > from.x && self.y == from.y
    }

    pub(crate) fn west_of(&self, from: &Position) -> bool {
        self.x < from.x && self.y == from.y
    }

    pub(crate) fn path_to(&self, other: &Position) -> (usize, usize, usize, usize) {
        (self.x, self.y, other.x, other.y)
    }
}

pub(crate) struct Distance {
    x: usize,
    y: usize,
}

impl Distance {
    pub(crate) fn new(x: usize, y: usize) -> Self {
        Distance { x, y }
    }

    pub(crate) fn x(&self) -> usize {
        self.x
    }

    pub(crate) fn y(&self) -> usize {
        self.y
    }
}

#[cfg(test)]
mod tests {
    use crate::models::position::Position;

    #[test]
    fn test_north_of() {
        assert!(Position::new(2, 0).north_of(&Position::new(2, 2)))
    }

    #[test]
    fn test_east_of() {
        assert!(Position::new(4, 2).east_of(&Position::new(2, 2)))
    }

    #[test]
    fn test_south_of() {
        assert!(Position::new(2, 4).south_of(&Position::new(2, 2)))
    }

    #[test]
    fn test_west_of() {
        assert!(Position::new(0, 2).west_of(&Position::new(2, 2)))
    }
}

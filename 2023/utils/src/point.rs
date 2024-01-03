use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    /// Manhattan distance
    ///
    /// ```
    /// use utils::point::Point;
    ///
    /// let a = Point::new(-1, 0);
    /// let b = Point::new(2, 8);
    /// assert_eq!(a.manhattan_distance(&b), 11);
    /// ```
    pub fn manhattan_distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

/*
** src/types/point.rs
*/

use std::cmp::Ordering;

/// Represents a point in a Cartesian coordinate plane
#[derive(Clone, Copy, Eq, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn is_origin(self) -> bool {
        self.x == 0 && self.y == 0
    }

    pub fn manhattan_distance(self) -> i32 {
        self.x.abs() + self.y.abs()
    }

    pub fn distance_to(self, other: Self) -> i32 {
        (other.x - self.x).abs() + (other.y - self.y).abs()
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        // points are ordered based on their x-coordinates
        // break ties using their y-coordinates
        if self.x == other.x {
            self.y.cmp(&other.y)
        } else {
            self.x.cmp(&other.x)
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

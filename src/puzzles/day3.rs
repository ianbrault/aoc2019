/*
** src/puzzles/day3.rs
*/

use crate::puzzles::Puzzle;
use crate::types::{Point, Wire};
use crate::utils::PuzzleInput;

pub struct Day3 {
    wire_1: Wire,
    wire_2: Wire,
}

impl Day3 {
    pub fn new() -> Self {
        let mut input = PuzzleInput::new(3);
        // convert input strings to wires
        Self {
            wire_1: Wire::from(input.next().unwrap()),
            wire_2: Wire::from(input.next().unwrap()),
        }
    }
}

impl Puzzle for Day3 {
    /// What is the Manhattan distance from the central port to the
    /// closest intersection?
    fn part_1(&self) -> i64 {
        // get all intersections, find the minimum Manhattan distance
        self.wire_1.intersections(&self.wire_2).into_iter()
            .map(Point::manhattan_distance)
            .min().unwrap() as i64
    }

    /// What is the fewest combined steps the wires must take to reach an
    /// intersection?
    fn part_2(&self) -> i64 {
        // get all intersections, find the minimum path length
        self.wire_1.intersections_path_lengths(&self.wire_2).iter()
            .map(|(w1_len, w2_len)| w1_len + w2_len)
            .min().unwrap() as i64
    }
}

/*
** src/puzzles/day3.rs
**/

use crate::puzzles::Puzzle;
use crate::types::{Point, Wire};
use crate::utils::PuzzleInput;


pub struct Day3;

impl Day3 {
    pub fn new() -> Self {
        Self { }
    }
}

impl Puzzle for Day3 {
    /// What is the Manhattan distance from the central port to the
    /// closest intersection?
    fn part_1(&self) -> i64 {
        let mut input = PuzzleInput::new(3);
        // convert input strings to wires
        let wire_1 = Wire::from(input.next().unwrap());
        let wire_2 = Wire::from(input.next().unwrap());

        // get all intersections, find the minimum Manhattan distance
        wire_1.intersections(&wire_2).iter()
            .map(Point::manhattan_distance)
            .min().unwrap() as i64
    }

    /// What is the fewest combined steps the wires must take to reach an
    /// intersection?
    fn part_2(&self) -> i64 {
        let mut input = PuzzleInput::new(3);
        // convert input strings to wires
        let wire_1 = Wire::from(input.next().unwrap());
        let wire_2 = Wire::from(input.next().unwrap());

        // get all intersections, find the minimum path length
        wire_1.intersections_path_lengths(&wire_2).iter()
            .map(|(w1_len, w2_len)| w1_len + w2_len)
            .min().unwrap() as i64
    }
}

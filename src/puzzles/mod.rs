/*
 * src/puzzles/mod.rs
 */

mod day1;
mod utils;

use day1::Day1;

/// trait object for daily Puzzles
pub trait Puzzle {
    fn part_1(&self) -> i64;
    fn part_2(&self) -> i64;
}

/// return a trait object corresponding to each puzzle to date
pub fn all_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(Day1::new()),
    ]
}
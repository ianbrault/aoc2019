/*
** src/puzzles/mod.rs
*/

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use day1::Day1;
use day2::Day2;
use day3::Day3;
use day4::Day4;
use day5::Day5;
use day6::Day6;
use day7::Day7;

/// trait object for daily Puzzles
pub trait Puzzle {
    fn part_1(&self) -> i64;
    fn part_2(&self) -> i64;
}

/// return a trait object corresponding to each puzzle to date
pub fn all_puzzles() -> Vec<Box<dyn Puzzle>> {
    vec![
        Box::new(Day1::new()),
        Box::new(Day2::new()),
        Box::new(Day3::new()),
        Box::new(Day4::new()),
        Box::new(Day5::new()),
        Box::new(Day6::new()),
        Box::new(Day7::new()),
    ]
}
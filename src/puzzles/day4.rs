/*
** src/puzzles/day4.rs
*/

use crate::puzzles::Puzzle;
use crate::types::Password;

pub struct Day4;

impl Day4 {
    pub fn new() -> Self {
        Self { }
    }
}

impl Puzzle for Day4 {
    /// Password criteria:
    /// (1) a six-digit number
    /// (2) within the range given in your puzzle input
    /// (3) two adjacent digits are the same (like 22 in 122345)
    /// (4) from left to right, the digits never decrease; they only ever
    ///     increase or stay the same (like 111123 or 135679)

    /// How many different passwords within the range given in your puzzle
    /// input meet these criteria?
    fn part_1(&self) -> i64 {
        // puzzle input
        let (lower, upper) = (171_309, 643_603);

        let passwords = Password::generate_in_range(lower, upper);
        // filter out all that do not contain any repeated digits
        passwords.into_iter()
            .filter(|p| p.contains_repeat())
            .count() as i64
    }

    /// Additional criteria: the two adjacent matching digits are not part of a
    /// larger group of matching digits.
    /// ex: 112233 meets these criteria because the digits never decrease and
    /// all repeated digits are exactly two digits long
    /// ex: 123444 no longer meets the criteria (the repeated 44 is part of a
    /// larger group of 444)
    /// ex: 111122 meets the criteria (even though 1 is repeated more than
    /// twice, it still contains a double 22)

    /// How many different passwords within the range given in your puzzle
    /// input meet all of the criteria?
    fn part_2(&self) -> i64 {
        // puzzle input
        let (lower, upper) = (171_309, 643_603);

        let passwords = Password::generate_in_range(lower, upper);
        // filter out all that do not contain any length-2 repeated digits
        passwords.into_iter()
            .filter(|p| p.contains_2repeat())
            .count() as i64
    }
}

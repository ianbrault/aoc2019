/*
 * src/puzzles/day1.rs
 */

use crate::puzzles::Puzzle;
use crate::puzzles::utils::{ParseIntIterExt, PuzzleInput};

pub struct Day1;

impl Day1 {
    pub fn new() -> Self {
        Self { }
    }

    // Fuel required to launch a given module is based on its mass.
    // Specifically, to find the fuel required for a module, take its mass,
    // divide by three, round down, and subtract 2.
    fn mass_to_fuel(mass: i64) -> i64 {
        (mass / 3) - 2
    }
}

// Fuel itself requires fuel just like a module - take its mass, divide by
// three, round down, and subtract 2. However, that fuel also requires fuel,
// and that fuel requires fuel, and so on. Any mass that would require negative
// fuel should instead be treated as if it requires zero fuel
//
// This can be easily modeled as an iterator: each iteration is another fuel
// calculation, and the iterator is exhausted when the next fuel calculation is
// zero or negative
struct FuelIterator {
    fuel: i64,
}

impl FuelIterator {
    fn new(mass: i64) -> Self {
        Self { fuel: mass }
    }
}

impl Iterator for FuelIterator {
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        self.fuel = Day1::mass_to_fuel(self.fuel);
        match self.fuel {
            n if n > 0 => Some(n),
            _ => None,
        }
    }
}

impl Puzzle for Day1 {
    /// What is the sum of the fuel requirements for all of the modules on
    /// your spacecraft?
    fn part_1(&self) -> i64 {
        PuzzleInput::new(1).as_ints()
            .map(Day1::mass_to_fuel).sum()
    }

    /// What is the sum of the fuel requirements for all of the modules on your
    /// spacecraft when also taking into account the mass of the added fuel?
    fn part_2(&self) -> i64 {
        PuzzleInput::new(1).as_ints()
            .map(|mass| FuelIterator::new(mass).sum::<i64>()).sum()
    }
}

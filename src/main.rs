/*
 * src/main.rs
 * Advent of Code 2019
 * as implemented in Rust by Ian Brault
 * see https://adventofcode.com/2019
 */

mod puzzles;

fn main() {
    println!("Advent of Code 2019 [Rust]");
    println!("by Ian Brault <ian.brault@engineering.ucla.edu>");

    for (day, puzzle) in puzzles::all_puzzles().iter().enumerate() {
        println!("\n=== Day {}", day + 1);
        println!("puzzle 1: {}", puzzle.part_1());
        println!("puzzle 2: {}", puzzle.part_2());
    }
}

/*
 * src/puzzles/day2.rs
 */

use crate::puzzles::Puzzle;
use crate::types::Intcode;
use crate::utils::PuzzleInput;

pub struct Day2;

impl Day2 {
    pub fn new() -> Self {
        Self { }
    }
}

impl Puzzle for Day2 {
    /// Once you have a working computer, the first step is to restore the
    /// gravity assist program to the "1202 program alarm" state it had just
    /// before the last computer caught fire. To do this, before running the
    /// program, replace position 1 with 12 and replace position 2 with 2. What
    /// value is left at position 0 after the program halts?
    fn part_1(&self) -> i64 {
        let input = PuzzleInput::new(2).next().unwrap();
        let mut prog = Intcode::new(Intcode::parse(input), 12, 2);
        prog.run()
    }

    /// Find the input noun and verb that cause the program to produce the
    /// output 19690720. What is 100 * noun + verb? (For example, if noun=12
    /// and verb=2, the answer would be 1202.)
    fn part_2(&self) -> i64 {
        let input = PuzzleInput::new(2).next().unwrap();
        let init_mem = Intcode::parse(input);

        'noun_loop: for noun in 0..100 {
            for verb in (0..100).rev() {
                let rc = Intcode::new(init_mem.clone(), noun, verb).run();
                if rc == 19690720 {
                    return (100 * noun + verb) as i64;
                } else if rc < 19690720 {
                    // the Intcode program is monotonically increasing
                    continue 'noun_loop;
                }
            }
        }
        // should never hit this point
        -1
    }
}


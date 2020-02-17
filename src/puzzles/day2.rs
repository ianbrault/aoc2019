/*
** src/puzzles/day2.rs
*/

use crate::puzzles::Puzzle;
use crate::types::Intcode;
use crate::utils::PuzzleInput;

pub struct Day2 {
    intcode_memory: Vec<i64>,
}

impl Day2 {
    pub fn new() -> Self {
        let input = PuzzleInput::new(2).next().unwrap();
        Self {
            intcode_memory: Intcode::parse(input),
        }
    }
}

impl Puzzle for Day2 {
    /// Once you have a working computer, the first step is to restore the
    /// gravity assist program to the "1202 program alarm" state it had just
    /// before the last computer caught fire. To do this, before running the
    /// program, replace position 1 with 12 and replace position 2 with 2. What
    /// value is left at position 0 after the program halts?
    fn part_1(&self) -> i64 {
        let mut prog = Intcode::new(self.intcode_memory.clone())
            .set_noun_verb(12, 2);
        prog.run();
        prog.memory[0]
    }

    /// Find the input noun and verb that cause the program to produce the
    /// output 19690720. What is 100 * noun + verb? (For example, if noun=12
    /// and verb=2, the answer would be 1202.)
    fn part_2(&self) -> i64 {
        'noun_loop: for noun in 0..100 {
            for verb in (0..100).rev() {
                let mut prog = Intcode::new(self.intcode_memory.clone())
                    .set_noun_verb(noun, verb);
                prog.run();
                if prog.memory[0] == 19_690_720 {
                    return (100 * noun + verb) as i64;
                } else if prog.memory[0] < 19_690_720 {
                    // the Intcode program is monotonically increasing
                    continue 'noun_loop;
                }
            }
        }
        panic!("end of loop reached, this should never happend")
    }
}


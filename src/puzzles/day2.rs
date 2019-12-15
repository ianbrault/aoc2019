/*
 * src/puzzles/day2.rs
 */

use crate::puzzles::Puzzle;
use crate::puzzles::intcode::Intcode;
use crate::puzzles::utils::PuzzleInput;

use std::thread;

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

        let mut handles = vec![];
        for tid in 0..4 {
            let loc_mem = init_mem.clone();
            // [FIXME] naive implementation:
            // spawn 4 threads and partition the input space between them
            handles.push(thread::spawn(move || {
                // nouns: 0..50 for thread 0, 1; 50..99 for thread 2, 3
                // verbs: 0..50 for thread 0, 2; 50..99 for thread 1, 3
                let nouns = ((tid / 2) * 50)..(50 + (tid / 2) * 49);
                let verbs = ((tid % 2) * 50)..(50 + (tid % 2) * 49);

                'noun_loop: for noun in nouns {
                    for verb in verbs.clone() {
                        let rc = Intcode::new(loc_mem.clone(), noun, verb).run();
                        if rc == 19690720 {
                            return Some((noun, verb));
                        } else if rc > 19690720 {
                            // this works because the Intcode program is
                            // monotonically increasing
                            continue 'noun_loop;
                        }
                    }
                }
                None
            }));
        }

        // join thread handles and get result
        for handle in handles {
            if let Some((noun, verb)) = handle.join().unwrap() {
                return (100 * noun + verb) as i64;
            }
        }
        // should never hit this point
        -1
    }
}


/*
** src/puzzles/day5.rs
*/

use crate::puzzles::Puzzle;
use crate::types::intcode::{self, Intcode};
use crate::utils::{self, PuzzleInput};

pub struct Day5 {
    // TEST diagnostic program
    program_memory: Vec<i64>,
}

impl Day5 {
    pub fn new() -> Self {
        let input = PuzzleInput::new(5).next().unwrap();
        Self {
            program_memory: Intcode::parse(input),
        }
    }
}

impl Puzzle for Day5 {
    /// After providing the air conditioner unit system ID (1) to the only
    /// input instruction and passing all the tests, what diagnostic code does
    /// the program produce?
    fn part_1(&self) -> i64 {
        let ac_unit_id = 1;
        let mut prog = Intcode::new(self.program_memory.clone())
            .with_input(ac_unit_id);

        prog.run();
        if prog.status != intcode::Status::Halted {
            panic!("program did not halt")
        }

        // validate outputs and get the diagnostic
        for (is_last, &out) in utils::is_last(prog.output_iter()) {
            if is_last {
                return out;
            } else if out != 0{
                panic!("program test returned {}", out)
            }
        }
        panic!("program produced no output")
    }

    /// What is the diagnostic code for system ID 5?
    fn part_2(&self) -> i64 {
        let thrm_rad_ctrl_id = 5;
        let mut prog = Intcode::new(self.program_memory.clone())
            .with_input(thrm_rad_ctrl_id);

        prog.run();
        if prog.status != intcode::Status::Halted {
            panic!("program did not halt")
        }

        // only provides the diagnostic code output
        if let Some(diagnostic_code) = prog.output() {
            diagnostic_code
        } else {
            panic!("program produced no output")
        }
    }
}


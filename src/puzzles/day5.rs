/*
** src/puzzles/day5.rs
*/

use crate::puzzles::Puzzle;
use crate::types::Intcode;
use crate::utils::PuzzleInput;

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
            .input(ac_unit_id);
        prog.run();

        // validate outputs and get the diagnostic
        if let Some((i, out)) = prog.validate_output() {
            panic!("program test {} returned {}", i, out)
        }
        prog.diagnostic_code()
    }

    /// What is the diagnostic code for system ID 5?
    fn part_2(&self) -> i64 {
        let thrm_rad_ctrl_id = 5;
        let mut prog = Intcode::new(self.program_memory.clone())
            .input(thrm_rad_ctrl_id);
        prog.run();

        // only provides the diagnostic code output, no need to validate others
        prog.diagnostic_code()
    }
}


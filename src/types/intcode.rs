/*
** src/types/intcode.rs
*/

use crate::utils::ParseIntIterExt;

type IntcodeInstr = (i64, i64, i64, i64);

/// An Intcode program is a list of integers separated by commas. To run one,
/// start by looking at the first integer (position 0). Here, you will find an
/// opcode - either 1, 2, or 99.
/// Opcode 1 adds together numbers read from two positions and stores the
/// result in a third position.
/// Opcode 2 works exactly like opcode 1, except it multiplies the two inputs.
/// Opcode 99 means that the program is finished and should immediately halt.
pub struct Intcode {
    memory: Vec<i64>,
    instr_ptr: usize,
}

impl Intcode {
    pub fn parse(prog_text: String) -> Vec<i64> {
        prog_text.split(",").map(String::from).as_ints().collect::<Vec<_>>()
    }

    pub fn new(mut memory: Vec<i64>, noun: usize, verb: usize) -> Self {
        memory[1] = noun as i64;
        memory[2] = verb as i64;
        Self { memory, instr_ptr: 0 }
    }

    pub fn get(&self, pos: i64) -> i64 {
        self.memory[pos as usize]
    }

    pub fn set(&mut self, pos: i64, val: i64) {
        self.memory[pos as usize] = val;
    }

    fn get_instr(&self) -> IntcodeInstr {
        (self.memory[self.instr_ptr],
         self.memory[self.instr_ptr + 1],
         self.memory[self.instr_ptr + 2],
         self.memory[self.instr_ptr + 3])
    }

    pub fn run(&mut self) -> i64 {
        // exhaust the iterator to get the final program state
        self.for_each(drop);
        self.memory[0]
    }
}

// Intcode execution can be easily modeled as an iterator, as it is simply a
// state machine. Each iteration reads the opcode at the current position and
// performs the indicated operation. The iterator is exhaused when opcode 99 is
// reached.
impl Iterator for Intcode {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        let (opcode, op1, op2, res) = self.get_instr();
        let ret = match opcode {
            1  => {
                self.set(res, self.get(op1) + self.get(op2));
                Some(())
            },
            2  => {
                self.set(res, self.get(op1) * self.get(op2));
                Some(())
            },
            99 => None,
            _  => panic!("{}: unknown opcode {}", self.instr_ptr, opcode),
        };
        self.instr_ptr += 4;
        ret
    }
}
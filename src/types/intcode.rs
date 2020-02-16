/*
** src/types/intcode.rs
*/

use crate::utils::ParseIntIterExt;

#[derive(Clone, Copy, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

impl From<i64> for ParameterMode {
    fn from(n: i64) -> Self {
        match n {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => panic!("invalid parameter mode {}", n),
        }
    }
}

struct IntcodeInstr {
    opcode: i64,
    param_modes: [ParameterMode; 3],
}

impl IntcodeInstr {
    fn decode(n: i64) -> Self {
        let opcode = n % 100;
        let param_modes = [
            ParameterMode::from((n / 100)   % 10),
            ParameterMode::from((n / 1000)  % 10),
            ParameterMode::from((n / 10000) % 10),
        ];

        Self { opcode, param_modes }
    }
}

/// An Intcode program is a list of integers separated by commas. To run one,
/// start by looking at the first integer (position 0). Here, you will find an
/// opcode. The instruction pointer is then moved past the opcode and its
/// parameters and execution continues.
pub struct Intcode {
    pub memory: Vec<i64>,
    instr_ptr: usize,
    input: Option<i64>,
    pub output: Vec<i64>,
}

impl Intcode {
    pub fn parse(prog_text: String) -> Vec<i64> {
        prog_text.split(",").as_ints().collect::<Vec<_>>()
    }

    pub fn new(memory: Vec<i64>) -> Self {
        Self { memory, instr_ptr: 0, input: None, output: vec![] }
    }

    pub fn set_noun_verb(mut self, noun: i64, verb: i64) -> Self {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self
    }

    /// adds input to an intcode program, following the builder pattern
    pub fn input(mut self, input: i64) -> Self {
        self.input = Some(input);
        self
    }

    fn get_param(&self, pn: usize) -> i64 {
        self.memory[self.instr_ptr + pn]
    }

    fn get(&self, param: i64, mode: ParameterMode) -> i64 {
        match mode {
            ParameterMode::Position => self.memory[param as usize],
            ParameterMode::Immediate => param,
        }
    }

    fn set(&mut self, pos: usize, val: i64) {
        self.memory[pos as usize] = val;
    }

    pub fn run(&mut self) {
        // exhaust the iterator to get the final program state
        self.for_each(drop);
    }

    /// validate that all outputs prior to the final output are zero
    /// if an output is non-zero, return the index and the output value
    pub fn validate_output(&self) -> Option<(usize, i64)> {
        let n_out = self.output.len() - 2;
        for (i, &out) in self.output[0..n_out].iter().enumerate() {
            if out != 0 {
                return Some((i, out));
            }
        }
        None
    }

    pub fn diagnostic_code(&self) -> i64 {
        self.output[self.output.len() - 1]
    }
}

// Intcode execution can be easily modeled as an iterator, as it is simply a
// state machine. Each iteration reads the opcode at the current position and
// performs the indicated operation. The iterator is exhaused when opcode 99 is
// reached.
impl Iterator for Intcode {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        // decode the instruction at the current instruction pointer
        let instr = IntcodeInstr::decode(self.memory[self.instr_ptr]);

        match instr.opcode {
            // opcode 1: add
            // 3 parameters
            // adds the opcodes at addresses specified by parameters 1 and 2
            // and stores the result at the address specified by parameter 3
            1  => {
                let op1 = self.get(self.get_param(1), instr.param_modes[0]);
                let op2 = self.get(self.get_param(2), instr.param_modes[1]);

                // address should be in position mode
                if instr.param_modes[2] == ParameterMode::Immediate {
                    panic!("opcode 1 received parameter 3 in immediate mode");
                }
                let addr = self.get_param(3) as usize;

                self.set(addr, op1 + op2);
                self.instr_ptr += 4;
            },
            // opcode 2: multiply
            // 3 parameters
            // adds the opcodes at addresses specified by parameters 1 and 2
            // and stores the result at the address specified by parameter 3
            2 => {
                let op1 = self.get(self.get_param(1), instr.param_modes[0]);
                let op2 = self.get(self.get_param(2), instr.param_modes[1]);

                // address should be in position mode
                if instr.param_modes[2] == ParameterMode::Immediate {
                    panic!("opcode 1 received parameter 3 in immediate mode");
                }
                let addr = self.get_param(3) as usize;

                self.set(addr, op1 * op2);
                self.instr_ptr += 4;
            },
            // opcode 3: input
            // 1 parameter
            // takes a single input and saves it to the address specified by
            // parameter 1
            3 => {
                if let Some(input) = self.input {
                    // address should be in position mode
                    if instr.param_modes[2] == ParameterMode::Immediate {
                        panic!("opcode 1 received parameter 3 in immediate mode");
                    }
                    let addr = self.get_param(1) as usize;

                    self.set(addr, input);
                    self.instr_ptr += 2;
                } else {
                    panic!("no input given to the program")
                }
            },
            // opcode 4: output
            // 1 parameter
            // outputs the value at the address specified by parameter 1
            4 => {
                let out_val = self.get(self.get_param(1), instr.param_modes[0]);
                self.output.push(out_val);
                self.instr_ptr += 2;
            },
            // opcode 5: jump-if-true
            // 2 parameters
            // if parameter 1 is non-zero, sets the instruction pointer to the
            // value from parameter 2; otherwise, does nothing
            5 => {
                let val  = self.get(self.get_param(1), instr.param_modes[0]);
                let addr = self.get(self.get_param(2), instr.param_modes[1]);

                if val != 0 {
                    self.instr_ptr = addr as usize;
                } else {
                    self.instr_ptr += 3;
                }
            },
            // opcode 6: jump-if-false
            // 2 parameters
            // if parameter 1 is zero, sets the instruction pointer to the
            // value from parameter 2; otherwise, does nothing
            6 => {
                let val  = self.get(self.get_param(1), instr.param_modes[0]);
                let addr = self.get(self.get_param(2), instr.param_modes[1]);

                if val == 0 {
                    self.instr_ptr = addr as usize;
                } else {
                    self.instr_ptr += 3;
                }
            },
            // opcode 7: less-than
            // 3 parameters
            // if parameter 1 is less than parameter 2, store 1 in the position
            // given by the third parameter; otherwise, store 0
            7 => {
                let op1 = self.get(self.get_param(1), instr.param_modes[0]);
                let op2 = self.get(self.get_param(2), instr.param_modes[1]);

                // address should be in position mode
                if instr.param_modes[2] == ParameterMode::Immediate {
                    panic!("opcode 1 received parameter 3 in immediate mode");
                }
                let addr = self.get_param(3) as usize;

                if op1 < op2 {
                    self.set(addr, 1);
                } else {
                    self.set(addr, 0);
                }

                self.instr_ptr += 4;
            },
            // opcode 8: equal
            // 3 parameters
            // if parameter 1 is equal to parameter 2, store 1 in the position
            // given by the third parameter; otherwise, store 0
            8 => {
                let op1 = self.get(self.get_param(1), instr.param_modes[0]);
                let op2 = self.get(self.get_param(2), instr.param_modes[1]);

                // address should be in position mode
                if instr.param_modes[2] == ParameterMode::Immediate {
                    panic!("opcode 1 received parameter 3 in immediate mode");
                }
                let addr = self.get_param(3) as usize;

                if op1 == op2 {
                    self.set(addr, 1);
                } else {
                    self.set(addr, 0);
                }

                self.instr_ptr += 4;
            },
            // opcode 99: no parameters
            // the program is finished and should immediately halt
            99 => {
                return None;
            },
            opcode @ _ => {
                panic!("{}: unknown opcode {}", self.instr_ptr, opcode)
            }
        }
        Some(())
    }
}
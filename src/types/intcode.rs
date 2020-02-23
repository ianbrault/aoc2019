/*
** src/types/intcode.rs
*/

use std::collections::VecDeque;
use std::fmt;

use crate::utils::ParseIntIterExt;

#[derive(PartialEq)]
pub enum Status {
    Initial,  // program has not yet started
    Running,  // program running
    Waiting,  // program is waiting for input
    Halted,   // program has halted
}

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

impl Into<i64> for ParameterMode {
    fn into(self) -> i64 {
        match self {
            ParameterMode::Position  => 0,
            ParameterMode::Immediate => 1,
        }
    }
}

struct Instr {
    opcode: i64,
    param_modes: [ParameterMode; 3],
}

impl Instr {
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

impl fmt::Debug for Instr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f, "{}{}{}{}",
            Into::<i64>::into(self.param_modes[2]),
            Into::<i64>::into(self.param_modes[1]),
            Into::<i64>::into(self.param_modes[0]),
            self.opcode)
    }
}

/// An Intcode program is a list of integers separated by commas. To run one,
/// start by looking at the first integer (position 0). Here, you will find an
/// opcode. The instruction pointer is then moved past the opcode and its
/// parameters and execution continues.
pub struct Intcode {
    pub memory: Vec<i64>,
    instr_ptr: usize,

    input:  VecDeque<i64>,
    output: VecDeque<i64>,

    pub status: Status,
}

impl Intcode {
    pub fn parse(prog_text: String) -> Vec<i64> {
        prog_text.split(',').as_ints().collect::<Vec<_>>()
    }

    pub fn new(memory: Vec<i64>) -> Self {
        Self {
            memory,
            instr_ptr: 0,
            input: VecDeque::new(),
            output: VecDeque::new(),
            status: Status::Initial,
        }
    }

    pub fn set_noun_verb(mut self, noun: i64, verb: i64) -> Self {
        self.memory[1] = noun;
        self.memory[2] = verb;
        self
    }

    /// adds input to an intcode program
    pub fn input(&mut self, input: i64) {
        self.input.push_back(input);
    }

    /// adds input to an intcode program, following the builder pattern
    pub fn with_input(mut self, input: i64) -> Self {
        self.input.push_back(input);
        self
    }

    /// pops an item from the output queue
    pub fn output(&mut self) -> Option<i64> {
        self.output.pop_front()
    }

    /// iterate through the output queue
    pub fn output_iter(&self) -> impl Iterator<Item=&i64> {
        self.output.iter()
    }

    fn get_param(&self, pn: usize, mode: ParameterMode) -> i64 {
        let param = self.memory[self.instr_ptr + pn];
        match mode {
            ParameterMode::Position => self.memory[param as usize],
            ParameterMode::Immediate => param,
        }
    }

    fn set(&mut self, pos: usize, val: i64) {
        self.memory[pos as usize] = val;
    }

    fn decode_instr(&self) -> Instr {
        Instr::decode(self.memory[self.instr_ptr])
    }

    pub fn run(&mut self) {
        // set the status to running
        self.status = Status::Running;

        while self.status == Status::Running {
            let instr = self.decode_instr();

            match instr.opcode {
                // opcode 1: add
                // 3 parameters
                // adds the values specified by parameters 1 and 2 and stores
                // the result at the address specified by parameter 3
                1  => {
                    let op1 = self.get_param(1, instr.param_modes[0]);
                    let op2 = self.get_param(2, instr.param_modes[1]);
                    // address should be in immediate mode
                    let addr = self.get_param(3, ParameterMode::Immediate);

                    self.set(addr as usize, op1 + op2);
                    self.instr_ptr += 4;
                },
                // opcode 2: multiply
                // 3 parameters
                // adds the values specified by parameters 1 and 2 and stores
                // the result at the address specified by parameter 3
                2 => {
                    let op1 = self.get_param(1, instr.param_modes[0]);
                    let op2 = self.get_param(2, instr.param_modes[1]);
                    // address should be in position mode
                    let addr = self.get_param(3, ParameterMode::Immediate);

                    self.set(addr as usize, op1 * op2);
                    self.instr_ptr += 4;
                },
                // opcode 3: input
                // 1 parameter
                // takes a single input and saves it to the address specified
                // by parameter 1
                3 => {
                    if let Some(input) = self.input.pop_front() {
                        // address should be in position mode
                        let addr = self.get_param(1, ParameterMode::Immediate);

                        self.set(addr as usize, input);
                        self.instr_ptr += 2;
                    } else {
                        // if there is no input, transition to waiting
                        self.status = Status::Waiting;
                    }
                },
                // opcode 4: output
                // 1 parameter
                // outputs the value at the address specified by parameter 1
                4 => {
                    let output = self.get_param(1, instr.param_modes[0]);
                    self.output.push_back(output);
                    self.instr_ptr += 2;
                },
                // opcode 5: jump-if-true
                // 2 parameters
                // if parameter 1 is non-zero, sets the instruction pointer to
                // the value from parameter 2; otherwise, does nothing
                5 => {
                    let val  = self.get_param(1, instr.param_modes[0]);
                    let addr = self.get_param(2, instr.param_modes[1]);

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
                    let val  = self.get_param(1, instr.param_modes[0]);
                    let addr = self.get_param(2, instr.param_modes[1]);

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
                    let op1 = self.get_param(1, instr.param_modes[0]);
                    let op2 = self.get_param(2, instr.param_modes[1]);
                    // address should be in position mode
                    let addr = self.get_param(3, ParameterMode::Immediate);

                    if op1 < op2 {
                        self.set(addr as usize, 1);
                    } else {
                        self.set(addr as usize, 0);
                    }

                    self.instr_ptr += 4;
                },
                // opcode 8: equal
                // 3 parameters
                // if parameter 1 is equal to parameter 2, store 1 in the position
                // given by the third parameter; otherwise, store 0
                8 => {
                    let op1 = self.get_param(1, instr.param_modes[0]);
                    let op2 = self.get_param(2, instr.param_modes[1]);
                    // address should be in position mode
                    let addr = self.get_param(3, ParameterMode::Immediate);

                    if op1 == op2 {
                        self.set(addr as usize, 1);
                    } else {
                        self.set(addr as usize, 0);
                    }

                    self.instr_ptr += 4;
                },
                // opcode 99: no parameters
                // the program is finished and should immediately halt
                99 => {
                    self.status = Status::Halted;
                },
                opcode => {
                    panic!("{}: unknown opcode {}", self.instr_ptr, opcode)
                }
            }
        }
    }
}

/// Used to chain multiple Intcode programs together
pub struct IntcodeChain {
    programs: Vec<Intcode>,
    feedback: bool,
    ipc: VecDeque<i64>,
}

impl IntcodeChain {
    /// sets the feedback loop
    pub fn with_feedback(mut self) -> Self {
        self.feedback = true;
        self
    }

    /// input is provided to the first program in the chain
    pub fn input(&mut self, input: i64) {
        self.programs[0].input(input);
    }

    pub fn output(&mut self) -> Option<i64> {
        let n_progs = self.programs.len();
        self.programs[n_progs - 1].output()
    }

    fn run_inner(&mut self) {
        for prog in self.programs.iter_mut() {
            // pump the IPC queue into the current program input
            while let Some(out) = self.ipc.pop_front() {
                prog.input(out);
            }

            prog.run();
            // unless in a feedback loop, ensure the program has halted
            if !self.feedback && prog.status != Status::Halted {
                panic!("program did not halt")
            }

            // add output to the IPC queue
            while let Some(out) = prog.output() {
                self.ipc.push_back(out);
            }
        }

        if self.feedback {
            // if the final program has not halted, feed the IPC queue output
            // into the first program and recurse
            let n_progs = self.programs.len();
            if self.programs[n_progs - 1].status != Status::Halted {
                while let Some(out) = self.ipc.pop_front() {
                    self.programs[0].input(out);
                }
                self.run_inner();
            }
        }
    }

    pub fn run(&mut self) {
        self.run_inner();

        // IPC queue should remain in the final program's output
        let n_progs = self.programs.len();
        while let Some(out) = self.ipc.pop_front() {
            self.programs[n_progs - 1].output.push_back(out);
        }
    }
}

impl<I> From<I> for IntcodeChain
where I: Iterator<Item=Intcode>
{
    fn from(it: I) -> Self {
        Self {
            programs: it.collect(),
            feedback: false,
            ipc: VecDeque::new(),
        }
    }
}

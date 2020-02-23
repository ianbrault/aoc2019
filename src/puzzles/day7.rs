/*
** src/puzzles/day7.rs
*/

use std::iter;

use crate::puzzles::Puzzle;
use crate::types::intcode::{Intcode, IntcodeChain};
use crate::utils::{Permutations, PuzzleInput};

pub struct Day7 {
    // Amplifier Control Software program
    amp_ctrl: Vec<i64>,
}

impl Day7 {
    pub fn new() -> Self {
        let input = PuzzleInput::new(7).next().unwrap();
        Self {
            amp_ctrl: Intcode::parse(input),
        }
    }
}

impl Puzzle for Day7 {
    /// Try every combination of phase settings on the amplifiers. What is the
    /// highest signal that can be sent to the thrusters?
    fn part_1(&self) -> i64 {
        let phase_settings = vec![0, 1, 2, 3, 4];
        let mut thruster_signals = vec![0; 120];

        for phase_seq in Permutations::new(&phase_settings) {
            // create the amplifier chain
            let mut amp_chain = IntcodeChain::from(
                iter::repeat_with(|| Intcode::new(self.amp_ctrl.clone()))
                    .zip(phase_seq)
                    .map(|(prog, &phase)| prog.with_input(phase)));

            // provide the initial input and run
            amp_chain.input(0);
            amp_chain.run();

            if let Some(out) = amp_chain.output() {
                thruster_signals.push(out);
            } else {
                panic!("program produced no output")
            }
        }

        thruster_signals.into_iter().max().unwrap()
    }

    /// Try every combination of the feedback phase settings on the amplifier
    /// feedback loop. What is the highest signal that can be sent to the
    /// thrusters?
    fn part_2(&self) -> i64 {
        let phase_settings = vec![5, 6, 7, 8, 9];
        let mut thruster_signals = vec![0; 120];

        for phase_seq in Permutations::new(&phase_settings) {
            // create the amplifier chain
            let mut amp_chain = IntcodeChain::from(
                iter::repeat_with(|| Intcode::new(self.amp_ctrl.clone()))
                    .zip(phase_seq)
                    .map(|(prog, &phase)| prog.with_input(phase))
            ).with_feedback();

            // provide the initial input and run
            amp_chain.input(0);
            amp_chain.run();

            if let Some(out) = amp_chain.output() {
                thruster_signals.push(out);
            } else {
                panic!("program produced no output")
            }
        }

        thruster_signals.into_iter().max().unwrap()
    }
}


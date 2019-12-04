/*
 * src/puzzles/utils.rs
 */

use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};
use std::path::Path;

pub struct PuzzleInput {
    inner: Lines<BufReader<File>>,
}

impl PuzzleInput {
    fn get_input_file(day: usize, part: usize) -> io::Result<File> {
        let filename = format!("d{}.p{}.input", day, part);
        let path = Path::new("..").join("..")
            .join("input").join(filename.as_str());

        File::open(path)
    }

    pub fn new(day: usize, part: usize) -> Self {
        let in_file = PuzzleInput::get_input_file(day, part).unwrap();
        Self { inner: BufReader::new(in_file).lines() }
    }

    pub fn as_ints(self) -> Box<dyn Iterator<Item=i64>> {
        Box::new(self.map(|line: io::Result<String>| {
            line.unwrap().parse::<i64>().unwrap()
        }))
    }
}

impl Iterator for PuzzleInput {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
}

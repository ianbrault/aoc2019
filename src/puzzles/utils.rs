/*
 * src/puzzles/utils.rs
 */

use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};
use std::path::Path;

type FileLines = Lines<BufReader<File>>;

pub struct PuzzleLines {
    inner: FileLines,
}

impl PuzzleLines {
    pub fn new(inner: FileLines) -> Self {
        Self { inner }
    }

    pub fn as_ints(self) -> Box<dyn Iterator<Item=i64>> {
        Box::new(self.inner.map(|line: io::Result<String>| {
            line.unwrap().parse::<i64>().unwrap()
        }))
    }
}

pub struct PuzzleInput {
    inner: BufReader<File>,
}

impl PuzzleInput {
    fn get_input_file(day: usize) -> io::Result<File> {
        let filename = format!("d{}.input", day);
        let path = Path::new("..").join("..")
            .join("input").join(filename.as_str());

        File::open(path)
    }

    pub fn new(day: usize) -> Self {
        let in_file = PuzzleInput::get_input_file(day).unwrap();
        Self { inner: BufReader::new(in_file) }
    }

    pub fn as_lines(self) -> PuzzleLines {
        PuzzleLines::new(self.inner.lines())
    }
}

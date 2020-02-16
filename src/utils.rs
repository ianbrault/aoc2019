/*
** src/utils.rs
*/

use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};
use std::path::Path;

type FileLines = Lines<BufReader<File>>;

pub struct PuzzleInput {
    inner: FileLines,
}

impl PuzzleInput {
    fn get_input_file(day: usize) -> io::Result<File> {
        let filename = format!("d{}.input", day);
        let path = Path::new("input").join(filename.as_str());
        File::open(path)
    }

    pub fn new(day: usize) -> Self {
        let in_file = PuzzleInput::get_input_file(day).unwrap();
        Self { inner: BufReader::new(in_file).lines() }
    }
}

impl Iterator for PuzzleInput {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(line) => Some(line.unwrap()),
            None => None,
        }
    }
}

pub struct ParseIntIter<I> {
    inner: I,
}

impl<I> ParseIntIter<I> {
    pub fn new(inner: I) -> Self {
        Self { inner }
    }
}

impl<I, S> Iterator for ParseIntIter<I>
where I: Iterator<Item=S>,
      S: Into<String>
{
    type Item = i64;

    fn next(&mut self) -> Option<Self::Item> {
        match self.inner.next() {
            Some(s) => Some(s.into().parse::<i64>().unwrap()),
            None => None
        }
    }
}

pub trait ParseIntIterExt: Iterator
where Self: Sized
{
    fn as_ints(self) -> ParseIntIter<Self> {
        ParseIntIter::new(self)
    }
}

impl<I: Iterator> ParseIntIterExt for I {}
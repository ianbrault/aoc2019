/*
** src/utils.rs
*/

use std::fs::File;
use std::io::{self, prelude::*, BufReader, Lines};
use std::iter;
use std::path::Path;

type FileLines = Lines<BufReader<File>>;

// an iterator over lines in an input file
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

// parses i64's out of an iterator over string-like items
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

// iterator extension for ParseIntIter
pub trait ParseIntIterExt: Iterator
where Self: Sized
{
    fn as_ints(self) -> ParseIntIter<Self> {
        ParseIntIter::new(self)
    }
}

impl<I: Iterator> ParseIntIterExt for I {}

// an iterator over all possible permutations of an input iterator
pub struct Permutations<'a, T> {
    // allocate the memory for returned permutations
    mem: Vec<&'a T>,
    first_iter: bool,
    // acts as the stack pointer
    i: usize,
    // encodes the stack state
    // c[k] is the for-loop counter for i = k + 1
    c: Vec<usize>,
}

impl<'a, T> Permutations<'a, T> {
    pub fn new(items: &'a [T]) -> Self {
        let mut mem = Vec::with_capacity(items.len());
        for el in items {
            mem.push(el);
        }

        Self {
            mem,
            first_iter: true,
            i: 0,
            c: vec![0; items.len()],
        }
    }
}

impl<'a, T> Iterator for Permutations<'a, T> {
    type Item=Vec<&'a T>;

    // iterator-adapted implementation of Heap's algorithm
    fn next(&mut self) -> Option<Self::Item> {
        if self.first_iter {
            // return the input with no swaps on the first iteration
            self.first_iter = false;
            Some(self.mem.clone())
        } else if self.i == self.mem.len() {
            // iteration is complete when the stack pointer hits the container length
            None
        } else if self.c[self.i] < self.i {
            if self.i % 2 == 0 {
                self.mem.swap(0, self.i);
            } else {
                self.mem.swap(self.c[self.i], self.i);
            }
            // increment the state, reset the stack pointer
            self.c[self.i] += 1;
            self.i = 0;
            Some(self.mem.clone())
        } else {
            // reset state, simulate stack pop by incrementing stack pointer
            self.c[self.i] = 0;
            self.i += 1;
            // recursive call to hit the if-condition
            self.next()
        }
    }
}

// for each item returned by the iterator, indicate whether or not it is the
// last item to be returned
// sourced from Kerollmops' iterator kit
// see https://gist.github.com/Kerollmops/5da7c03b6601d63b4345173f895756a6
pub fn is_last<I>(iter: I) -> impl Iterator<Item=(bool, I::Item)>
where I: IntoIterator
{
    let mut iter = iter.into_iter().peekable();
    iter::from_fn(move || {
        iter.next().map(|item| (iter.peek().is_none(), item))
    })
}

// clump an iterator into uniform-sized groups
pub fn clump<I>(iter: I, n: usize) -> impl Iterator<Item=Vec<I::Item>>
where I: IntoIterator
{
    let mut iter = iter.into_iter().peekable();
    iter::from_fn(move || {
        let mut an = 0;
        let mut acc = vec![];
        #[allow(clippy::while_let_on_iterator)]
        while let Some(el) = iter.next() {
            acc.push(el);
            an += 1;
            if an == n {
                break;
            }
        }

        if acc.is_empty() {
            None
        } else {
            Some(acc)
        }
    })
}

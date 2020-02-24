/*
** src/puzzles/day8.rs
*/

#![allow(clippy::naive_bytecount)]

use crate::puzzles::Puzzle;
use crate::types::SIFImage;
use crate::utils::{self, PuzzleInput};

pub struct Day8 {
    image: SIFImage,
}

impl Day8 {
    pub fn new() -> Self {
        // INPUT: the image you received is 25 pixels wide and 6 pixels tall
        let (width, height) = (25, 6);

        let transmission = PuzzleInput::new(8).next().unwrap();
        // decode the transmission into an SIF-encoded image
        let image = SIFImage::decode(transmission, width, height);

        Self { image }
    }

    fn display_image(layer: Vec<u8>) {
        let width = 25;  // from input
        for row in utils::clump(layer.into_iter(), width) {
            print!("█");
            for pixel in row {
                if pixel == 0 {
                    print!("█");
                } else {
                    print!(" ");
                }
            }
            println!();
        }
    }
}

impl Puzzle for Day8 {
    /// find the layer that contains the fewest 0 digits. On that layer, what
    /// is the number of 1 digits multiplied by the number of 2 digits?
    fn part_1(&self) -> i64 {
        // find the layer with the fewest 0 digits
        let (layer, _) = self.image.layers()
            .map(|layer| (layer, layer.iter().filter(|&&n| n == 0).count()))
            .min_by_key(|(_, count)| *count)
            .unwrap();

        // return the number of 1 digits multiplied by the number of 2 digits
        let ones = layer.iter().filter(|&&n| n == 1).count();
        let twos = layer.iter().filter(|&&n| n == 2).count();

        (ones * twos) as i64
    }

    /// What message is produced after decoding your image?
    fn part_2(&self) -> i64 {
        // render the image into a single layer
        let render = self.image.render();

        // ANSWER: EJRGP
        Self::display_image(render);

        0
    }
}

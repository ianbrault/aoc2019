/*
** src/types/sif.rs
*/

use crate::utils;

type SIFLayer = Vec<u8>;

/// Space Image Format encoded images are a series of digits that represent the
/// color of a single pixel. Digits fill each row of the image left-to-right,
/// then move downward to the next row, filling rows top-to-bottom until every
/// pixel of the image is filled. Each image consists of a series of
/// identically-sized layers that are filled in this way.
pub struct SIFImage {
    width: usize,
    height: usize,
    layers: Vec<SIFLayer>,
}

impl SIFImage {
    pub fn decode(s: String, width: usize, height: usize) -> Self {
        // decode the string into pixels
        let pixels = s.chars().map(|c| c.to_digit(10).unwrap() as u8);
        // collect pixels into layers
        let layers = utils::clump(pixels, width * height).collect();

        Self { width, height, layers }
    }

    pub fn layers(&self) -> impl Iterator<Item=&SIFLayer> {
        self.layers.iter()
    }

    fn top_visible_pixel(&self, pixn: usize) -> u8 {
        // select the first non-transparent pixel
        let pixel = self.layers()
            .map(|layer| layer[pixn])
            .find(|&pixel| pixel != 2);

        if let Some(pixel) = pixel {
            // non-transparent pixel found
            pixel
        } else {
            // otherwise all layers are transparent
            2
        }
    }

    pub fn render(&self) -> SIFLayer {
        let size = self.width * self.height;
        let mut layer = vec![2; size];

        for (pixn, pixel) in layer.iter_mut().enumerate() {
            *pixel = self.top_visible_pixel(pixn);
        }

        layer
    }
}

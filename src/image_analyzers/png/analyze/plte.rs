use crate::image_analyzers::png::PngImage;
use crate::image_analyzers::png::chunk::PngChunk;

use log::{error};
use crate::types::color::Color;

pub fn analyze_plte_chunk(png_chunk: &PngChunk, png_image: &mut PngImage) {
    if png_chunk.get_length() % 3 != 0 {
        error!("PLTE Chunk data length was not divisible by 3! PNG possibly corrupted!");
        panic!("PNG possibly corrupted.");
    }
    let chunk_data = png_chunk.get_data();

    let mut byte_index: usize = 0;

    let byte_length = chunk_data.len();

    while byte_index < byte_length {
        let red_val = *chunk_data.get(byte_index)
            .expect("Ran over data in PLTE chunk, PNG may be corrupted.");
        byte_index += 1;
        let green_val = *chunk_data.get(byte_index)
            .expect("Ran over data in PLTE chunk, PNG may be corrupted.");
        byte_index += 1;
        let blue_val = *chunk_data.get(byte_index)
            .expect("Ran over data in PLTE chunk, PNG may be corrupted.");
        byte_index += 1;

        png_image.push_palette_entry(Color::new(red_val, green_val, blue_val));
    }
}
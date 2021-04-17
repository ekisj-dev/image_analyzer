use crate::image_analyzers::png::chunk::PngChunk;
use crate::image_analyzers::png::PngImage;

use log::{debug, trace, warn, error};

use std::io::Read;
use crate::image_analyzers::Pixel;
use flate2::read::ZlibDecoder;
use truecolor::pixels_from_truecolor;

mod truecolor;
mod filter;

pub fn analyze_idat_chunk(idat_chunk: &PngChunk, png_image: &mut PngImage) {

    let chunk_data = idat_chunk.get_data();

    let mut decompressed = Vec::new();

    let mut d = ZlibDecoder::new(chunk_data.as_slice());

    d.read_to_end(&mut decompressed).expect("Image decompression failed!");

    debug!("Decompressed Bytes: {}", decompressed.len());

    let full_pixels: Vec<Vec<Pixel>> = match png_image.get_color_type() {
        2 => pixels_from_truecolor(png_image, &decompressed, false),
        6 => pixels_from_truecolor(png_image, &decompressed, true),
        _ => Vec::new()
    };

    debug!("Number of scanlines: {}", full_pixels.len());

    png_image.set_pixels(full_pixels);
}




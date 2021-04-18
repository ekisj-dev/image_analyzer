use crate::image_analyzers::png::chunk::PngChunk;
use crate::image_analyzers::png::PngImage;

use log::{debug};

use std::io::Read;
use flate2::read::ZlibDecoder;
use truecolor::pixels_from_truecolor;
use crate::types::image::Pixel;
use crate::image_analyzers::png::analyze::idat::indexed::pixels_from_indexed;
use crate::image_analyzers::png::analyze::idat::grayscale::pixels_from_grayscale;

mod grayscale;
mod truecolor;
mod indexed;
mod filter;

pub fn analyze_idat_chunk(idat_chunk: &PngChunk, png_image: &mut PngImage) {

    let chunk_data = idat_chunk.get_data();

    let mut decompressed = Vec::new();

    let mut d = ZlibDecoder::new(chunk_data.as_slice());

    d.read_to_end(&mut decompressed).expect("Image decompression failed!");

    debug!("Decompressed Bytes: {}", decompressed.len());

    let full_pixels: Vec<Vec<Pixel>> = match png_image.get_color_type() {
        0 => pixels_from_grayscale(png_image, &decompressed, false),
        2 => pixels_from_truecolor(png_image, &decompressed, false),
        3 => pixels_from_indexed(png_image, &decompressed),
        4 => pixels_from_grayscale(png_image, &decompressed, true),
        6 => pixels_from_truecolor(png_image, &decompressed, true),
        _ => Vec::new()
    };

    debug!("Number of scanlines: {}", full_pixels.len());

    png_image.set_pixels(full_pixels);
}




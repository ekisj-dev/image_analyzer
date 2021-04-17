use crate::image_analyzers::png::chunk::PngChunk;
use crate::image_analyzers::png::PngImage;

use log::{debug, trace};

use compress::zlib;
use std::io::Read;
use crate::image_analyzers::Pixel;

pub fn analyze_idat_chunk(idat_chunk: &PngChunk, png_image: &mut PngImage) {

    let chunk_data = idat_chunk.get_data();

    let mut decompressed = Vec::new();

    zlib::Decoder::new(chunk_data.as_slice()).read_to_end(&mut decompressed)
        .expect("Image decompression failed!");

    debug!("Decompressed Bytes: {:?}", decompressed);

    let full_pixels: Vec<Vec<Pixel>> = match png_image.get_color_type() {
        2 => pixels_from_truecolor(png_image, &decompressed, false),
        6 => pixels_from_truecolor(png_image, &decompressed, true),
        _ => Vec::new()
    };

    debug!("Number of scanlines: {}", full_pixels.len());

    png_image.set_pixels(full_pixels);
}

fn pixels_from_truecolor(png_image: &PngImage, decompressed: &Vec<u8>, with_alpha: bool) -> Vec<Vec<Pixel>> {
    let mut byte_index: usize = 0;
    let decompressed_size = decompressed.len();

    let mut full_pixels: Vec<Vec<Pixel>> = Vec::new();

    while byte_index < decompressed_size {
        trace!("New scanline...");
        let mut scanline: Vec<Pixel> = Vec::new();

        // Unused
        let _filter_type = decompressed.get(byte_index).unwrap_or(&0);

        byte_index += 1;

        for _ in 0..png_image.get_width().clone() {
            trace!("Grabbing pixel...");
            let red_val = decompressed.get(byte_index).unwrap_or(&0).clone();
            byte_index += 1;
            let green_val = decompressed.get(byte_index).unwrap_or(&0).clone();
            byte_index += 1;
            let blue_val = decompressed.get(byte_index).unwrap_or(&0).clone();
            byte_index += 1;

            let alpha_val: u8;

            if with_alpha {
                alpha_val = decompressed.get(byte_index).unwrap_or(&u8::MAX).clone();
                byte_index += 1;
            } else {
                alpha_val = u8::MAX;
            }

            scanline.push(Pixel::new(red_val, green_val, blue_val, alpha_val));
        }

        trace!("Scanline generated!");

        if scanline.is_empty() {
            trace!("Scanline was empty! Breaking!");
            break;
        }

        trace!("Pushing scanline.");
        full_pixels.push(scanline);
    }

    full_pixels
}
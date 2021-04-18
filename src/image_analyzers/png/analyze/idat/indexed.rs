use crate::image_analyzers::png::PngImage;
use crate::types::image::Pixel;

use log::{warn};
use crate::types::color::Color;

pub fn pixels_from_indexed(png_image: &PngImage, decompressed: &Vec<u8>) -> Vec<Vec<Pixel>> {
    let image_width: usize = png_image.get_width().clone() as usize;

    let mut byte_index: usize = 0;
    let decompressed_size = decompressed.len();

    let mut full_pixels: Vec<Vec<Pixel>> = Vec::new();

    while byte_index < decompressed_size {
        let mut scanline: Vec<Pixel> = Vec::new();

        // Unused
        let _filter_type = decompressed.get(byte_index).unwrap_or(&0);

        byte_index += 1;

        for _ in 0..image_width {

            let palette_index: usize = *(decompressed.get(byte_index)
                .expect("Failed to retrieve byte from decompressed data."))
                as usize;

            byte_index += 1;

            let palette_entry: &Color = png_image.get_palette_entry(palette_index);

            let red_val = palette_entry.red();
            let green_val = palette_entry.green();
            let blue_val = palette_entry.blue();

            let new_pixel = Pixel::new(red_val, green_val, blue_val, u8::MAX);

            scanline.push(new_pixel);
        }

        if scanline.is_empty() {
            warn!("Scanline was empty! Breaking!");
            break;
        }

        full_pixels.push(scanline);
    }

    full_pixels
}
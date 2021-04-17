use crate::image_analyzers::png::PngImage;
use crate::image_analyzers::Pixel;
use super::filter::apply_filter;

use log::{debug, trace, warn, error};

const PIXEL_SIZE_IN_BYTES: u8 = 4;

pub fn pixels_from_truecolor(png_image: &PngImage, decompressed: &Vec<u8>, with_alpha: bool) -> Vec<Vec<Pixel>> {
    let image_width: usize = png_image.get_width().clone() as usize;
    let scanline_width: usize = image_width + 1;

    let mut byte_index: usize = 0;
    let decompressed_size = decompressed.len();

    let mut full_pixels: Vec<Vec<Pixel>> = Vec::new();

    let mut prior_scanline: Vec<Pixel> = Vec::new();

    while byte_index < decompressed_size {
        let mut scanline: Vec<Pixel> = Vec::new();

        // Unused
        let filter_type = decompressed.get(byte_index).unwrap_or(&0);

        trace!("Found a filter type of {}", filter_type);

        byte_index += 1;

        for width_index in 0..image_width {
            let prior_pixel;
            if width_index <= 0 {
                prior_pixel = Pixel::empty();
            } else {
                prior_pixel = scanline.get(width_index as usize - 1).unwrap().clone();
            }

            let upper_pixel;
            if prior_scanline.is_empty() {
                upper_pixel = Pixel::empty();
            } else {
                upper_pixel = prior_scanline.get(width_index).unwrap().clone();
            }

            let upper_left_pixel;
            if prior_scanline.is_empty() || width_index <= 0 {
                upper_left_pixel = Pixel::empty();
            } else {
                upper_left_pixel = prior_scanline.get(width_index as usize - 1).unwrap().clone();
            }

            let red_val = decompressed.get(byte_index).
                unwrap_or_else(|| {
                    error!("Error encountered when reading red value");
                    &0
                }).clone();
            let red_val = apply_filter(filter_type, red_val, *prior_pixel.red(), *upper_pixel.red(), *upper_left_pixel.red());
            byte_index += 1;

            let green_val = decompressed.get(byte_index)
                .unwrap_or_else(|| {
                    error!("Error encountered when reading green value");
                    &0
                }).clone();
            let green_val = apply_filter(filter_type, green_val, *prior_pixel.green(), *upper_pixel.green(), *upper_left_pixel.green());
            byte_index += 1;

            let blue_val = decompressed.get(byte_index)
                .unwrap_or_else(|| {
                    error!("Error encountered when reading blue value");
                    &0
                }).clone();
            let blue_val = apply_filter(filter_type, blue_val, *prior_pixel.blue(), *upper_pixel.blue(), *upper_left_pixel.blue());
            byte_index += 1;

            let alpha_val: u8;

            if with_alpha {
                let alpha_temp = decompressed.get(byte_index).unwrap_or_else(|| {
                    error!("Error encountered when reading alpha value");
                    &u8::MAX
                }).clone();
                alpha_val = apply_filter(filter_type, alpha_temp, *prior_pixel.alpha(), *upper_pixel.alpha(), *upper_left_pixel.alpha());

                byte_index += 1;
            } else {
                alpha_val = u8::MAX;
            }

            let new_pixel = Pixel::new(red_val, green_val, blue_val, alpha_val);

            scanline.push(new_pixel);
        }

        if scanline.is_empty() {
            warn!("Scanline was empty! Breaking!");
            break;
        }

        prior_scanline = scanline.clone();

        full_pixels.push(scanline);
    }

    full_pixels
}
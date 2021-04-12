use crate::image_analyzers::png::chunk::PngChunk;
use crate::image_analyzers::png::PngImage;
use crate::image_analyzers::png::PngUnit;
use crate::math_utils::as_u32_be;

use std::convert::TryFrom;

pub fn analyze_phys_chunk(phys_chunk: &PngChunk, png_image: &mut PngImage) {
    println!("Analyzing pHYs chunk...");
    let chunk_data = phys_chunk.get_data();

    let ppu_x = as_u32_be(<&[u8; 4]>::try_from(&chunk_data[..4]).unwrap());

    println!("Found {} pixels per unit, in the X-direction.", ppu_x);

    png_image.set_pixels_per_unit_x(ppu_x);

    let ppu_y = as_u32_be(<&[u8; 4]>::try_from(&chunk_data[4..8]).unwrap());

    println!("Found {} pixels per unit, in the Y-direction.", ppu_y);

    png_image.set_pixels_per_unit_y(ppu_y);

    let unit_specifier = chunk_data.get(8).unwrap_or(&0);

    let unit_specifier = PngUnit::from(unit_specifier);

    println!("Found a unit specifier of {:?}", &unit_specifier);

    png_image.set_pixels_per_unit_specifier(unit_specifier);

    println!("Finished analyzing pHYs chunk!");
}
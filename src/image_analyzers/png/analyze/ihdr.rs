use crate::image_analyzers::png::chunk::PngChunk;
use crate::image_analyzers::png::PngImage;
use crate::math_utils::as_u32_be;

use log::{debug, trace};

use std::convert::TryFrom;

pub fn analyze_ihdr_chunk(ihdr_chunk: &PngChunk, png_image: &mut PngImage) {
    debug!("Analyzing IHDR chunk...");
    let chunk_data = ihdr_chunk.get_data();
    let width: u32 = as_u32_be(<&[u8; 4]>::try_from(&chunk_data[0..4]).unwrap());
    trace!("Found a width of {}...", width);
    png_image.set_width(width);
    let height: u32 = as_u32_be(<&[u8; 4]>::try_from(&chunk_data[4..8]).unwrap());
    trace!("Found a height of {}...", height);
    png_image.set_height(height);

    let bit_depth = chunk_data.get(8).expect("Unable to parse bit depth.");
    let color_type = chunk_data.get(9).expect("Unable to parse color type.");

    png_image.set_bit_depth(*bit_depth);
    png_image.set_color_type(*color_type);

    let compression_method = chunk_data.get(10)
        .expect("Unable to parse compression method.");
    let filter_method = chunk_data.get(11)
        .expect("Unable to parse filter method.");
    let interlace_method = chunk_data.get(12)
        .expect("Unable to parse interlace method.");

    png_image.set_compression_method(*compression_method);
    png_image.set_filter_method(*filter_method);
    png_image.set_interlace_method(*interlace_method);

    trace!("...and found the following metadata:");
    trace!("Bit Depth: {}, Color Type: {}", bit_depth, color_type);
    trace!("Compression Method: {}, Filter Method: {}, Interlace Method: {}",
             compression_method, filter_method, interlace_method);

    debug!("Finished analyzing IHDR chunk!");
}
use super::chunk::{PngChunk, ChunkType};
use super::{PngImage};

use log::{warn};

mod ihdr;
mod sbit;
mod phys;
mod idat;
mod plte;

pub fn analyze_chunk(png_chunk: &PngChunk, png_image: &mut PngImage) {
    match png_chunk.get_type() {
        ChunkType::IHDR => ihdr::analyze_ihdr_chunk(png_chunk, png_image),
        ChunkType::sBIT => sbit::analyze_sbit_chunk(png_chunk, png_image),
        ChunkType::pHYs => phys::analyze_phys_chunk(png_chunk, png_image),
        ChunkType::IDAT => idat::analyze_idat_chunk(png_chunk, png_image),
        ChunkType::PLTE => plte::analyze_plte_chunk(png_chunk, png_image),
        _ => warn!("Ignoring chunk with type {:?} for now...", png_chunk.get_type())
    }
}
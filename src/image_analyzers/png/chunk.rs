use std::str::FromStr;
use std::convert::{TryFrom, Infallible};
use crate::math_utils::as_u32_be;

pub struct PngChunk {
    length: u32,
    c_type: ChunkType,
    c_data: Vec<u8>,
    crc: [u8; 4],
}

impl PngChunk {
    pub fn create_chunk(length: u32, c_type: ChunkType, c_data: Vec<u8>, crc: [u8; 4]) -> PngChunk {
        PngChunk {
            length,
            c_type,
            c_data,
            crc
        }
    }

    pub fn get_length(&self) -> &u32 {
        &self.length
    }

    pub fn get_type(&self) -> &ChunkType {
        &self.c_type
    }

    pub fn get_data(&self) -> &Vec<u8> {
        &self.c_data
    }

    pub fn validate_data(&self) -> bool {
        // Do work to validate data using CRC
        as_u32_be(&self.crc) != 0
    }
}

#[derive(Debug)]
#[allow(non_camel_case_types)] // Matches the PNG specifications.
pub enum ChunkType {
    IHDR, // Image Header
    PLTE, // Palette
    IDAT, // Image Data
    IEND, // Image Trailer
    cHRM, // Primary Chromaticities
    gAMA, // Image Gamma
    iCCP, // Embedded ICC Profile
    sBIT, // Significant Bits
    sRGB, // Standard RGB color space
    bKGD, // Background Color
    hIST, // Palette Histogram
    tRNS, // Transparency
    pHYs, // Physical Pixel Dimensions
    sPLT, // Suggested Palette
    tIME, // Image Last-Modification Time
    iTXt, // International Textual Data
    tEXt, // Textual Data
    zTXt, // Compressed Textual Data
    UNKN([u8; 4]) // Unknown Type
}

impl FromStr for ChunkType {
    type Err = Infallible;

    fn from_str(input: &str) -> Result<ChunkType, Self::Err> {
        match input {
            "IHDR" => Ok(ChunkType::IHDR),
            "PLTE" => Ok(ChunkType::PLTE),
            "IDAT" => Ok(ChunkType::IDAT),
            "IEND" => Ok(ChunkType::IEND),
            "cHRM" => Ok(ChunkType::cHRM),
            "gAMA" => Ok(ChunkType::gAMA),
            "iCCP" => Ok(ChunkType::iCCP),
            "sBIT" => Ok(ChunkType::sBIT),
            "sRGB" => Ok(ChunkType::sRGB),
            "bKGD" => Ok(ChunkType::bKGD),
            "hIST" => Ok(ChunkType::hIST),
            "tRNS" => Ok(ChunkType::tRNS),
            "pHYs" => Ok(ChunkType::pHYs),
            "sPLT" => Ok(ChunkType::sPLT),
            "tIME" => Ok(ChunkType::tIME),
            "iTXt" => Ok(ChunkType::iTXt),
            "tEXt" => Ok(ChunkType::tEXt),
            "zTXt" => Ok(ChunkType::zTXt),
            _ => Ok(ChunkType::UNKN(<[u8; 4]>::try_from(input.as_bytes().clone()).unwrap()))
        }
    }
}
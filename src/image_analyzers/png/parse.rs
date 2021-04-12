use super::chunk::{ChunkType, PngChunk};
use std::str::FromStr;
use crate::math_utils::as_u32_be;
use std::convert::TryFrom;
use std::string::FromUtf8Error;
use std::error::Error;

const DATA_LENGTH_BYTE_SIZE: usize = 4;
const CHUNK_TYPE_BYTE_SIZE: usize = 4;
const CRC_BYTE_SIZE: usize = 4;

fn remove_header(image_bytes: &Vec<u8>) -> &[u8] {
    return &image_bytes[8..];
}

fn get_chunk_type(chunk_type: &[u8]) -> Result<ChunkType, FromUtf8Error> {
    // ChunkType::from_str cannot currently throw Err
    Ok(
        ChunkType::from_str(String::from_utf8(Vec::from(chunk_type))?.as_str())
            .expect("Chunk Type unable to be decoded!")
    )
}

pub fn collect_chunks(image_bytes: Vec<u8>) -> Result<Vec<PngChunk>, Box<dyn Error>> {
    let cleaned_bytes = remove_header(&image_bytes);

    let mut png_chunks: Vec<PngChunk> = Vec::new();

    let mut cur_index: usize = 0;

    while cleaned_bytes.len() > cur_index {
        // Parse Data Length
        let data_length: u32 =
            as_u32_be(<&[u8; DATA_LENGTH_BYTE_SIZE]>::try_from(
                &cleaned_bytes[cur_index..(cur_index + DATA_LENGTH_BYTE_SIZE)]
            )?);
        cur_index += DATA_LENGTH_BYTE_SIZE;

        // Parse Chunk Type
        let chunk_type: [u8; CHUNK_TYPE_BYTE_SIZE] =
            *<&[u8; CHUNK_TYPE_BYTE_SIZE]>::try_from(
                &cleaned_bytes[cur_index..(cur_index + CHUNK_TYPE_BYTE_SIZE)]
            )?;
        let chunk_type = get_chunk_type(&chunk_type)?;
        cur_index += CHUNK_TYPE_BYTE_SIZE;

        // Parse Chunk Data
        let chunk_data: Vec<u8> =
            Vec::from(&cleaned_bytes[cur_index..(cur_index + data_length as usize)]);
        cur_index += data_length as usize;

        // Parse CRC Bytes
        let crc: [u8; CRC_BYTE_SIZE] =
            *<&[u8; CRC_BYTE_SIZE]>::try_from(
                &cleaned_bytes[cur_index..(cur_index + CRC_BYTE_SIZE)]
            )?;
        cur_index += CRC_BYTE_SIZE;

        png_chunks.push(
            PngChunk::create_chunk(data_length, chunk_type, chunk_data, crc)
        );
    }

    Ok(png_chunks)
}
use crate::image_analyzers::png::PngUnit::{METER, UNKN};

pub mod chunk;
pub mod parse;
pub mod analyze;

#[derive(Debug)]
pub enum PngUnit {
    METER,
    UNKN
}

impl From<&u8> for PngUnit {
    fn from(byte: &u8) -> Self {
        match byte {
            1 => METER,
            _ => UNKN
        }
    }
}

pub struct PngImage {
    width: u32,
    height: u32,
    color_type: u8,
    bit_depth: u8,
    // TODO: Add Significant Bits per Channel, based on channels
    // Consider that images can have a few different numbers of channels
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
    pixels_per_unit_x: u32,
    pixels_per_unit_y: u32,
    pixels_per_unit_specifier: PngUnit
}

impl PngImage {
    pub fn create_image() -> PngImage {
        PngImage {
            width: 0,
            height: 0,
            color_type: 0,
            bit_depth: 0,
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
            pixels_per_unit_x: 0,
            pixels_per_unit_y: 0,
            pixels_per_unit_specifier: PngUnit::UNKN
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn get_width(&self) -> &u32 {
        &self.width
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn get_height(&self) -> &u32 {
        &self.height
    }

    pub fn set_color_type(&mut self, color_type: u8) {
        self.color_type = color_type;
    }

    pub fn get_color_type(&self) -> &u8 {
        &self.color_type
    }

    pub fn set_bit_depth(&mut self, bit_depth: u8) {
        self.bit_depth = bit_depth;
    }

    pub fn get_bit_depth(&self) -> &u8 {
        &self.bit_depth
    }

    pub fn set_compression_method(&mut self, compression_method: u8) {
        self.compression_method = compression_method;
    }

    pub fn get_compression_method(&self) -> &u8 {
        &self.compression_method
    }

    pub fn set_filter_method(&mut self, filter_method: u8) {
        self.filter_method = filter_method;
    }

    pub fn get_filter_method(&self) -> &u8 {
        &self.filter_method
    }

    pub fn set_interlace_method(&mut self, interlace_method: u8) {
        self.interlace_method = interlace_method;
    }

    pub fn get_interlace_method(&self) -> &u8 {
        &self.interlace_method
    }

    pub fn set_pixels_per_unit_x(&mut self, pixels_per_unit_x: u32) {
        self.pixels_per_unit_x = pixels_per_unit_x;
    }

    pub fn get_pixels_per_unit_x(&self) -> u32 {
        self.pixels_per_unit_x
    }

    pub fn set_pixels_per_unit_y(&mut self, pixels_per_unit_y: u32) {
        self.pixels_per_unit_y = pixels_per_unit_y;
    }

    pub fn get_pixels_per_unit_y(&self) -> u32 {
        self.pixels_per_unit_y
    }

    pub fn set_pixels_per_unit_specifier(&mut self, pixels_per_unit_specifier: PngUnit) {
        self.pixels_per_unit_specifier = pixels_per_unit_specifier;
    }

    pub fn get_pixels_per_unit_specifier(&self) -> &PngUnit {
        &self.pixels_per_unit_specifier
    }
}
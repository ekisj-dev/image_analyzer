use crate::types::image::Pixel;
use crate::types::color::Color;

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
            1 => PngUnit::METER,
            _ => PngUnit::UNKN
        }
    }
}

pub enum PngChannel {
    RED(u8),
    BLUE(u8),
    GREEN(u8),
    ALPHA(u8),
    GRAY(u8)
}

pub struct PngImage {
    width: u32,
    height: u32,
    color_type: u8,
    bit_depth: u8,
    png_channels: Vec<PngChannel>,
    palette: Vec<Color>,
    compression_method: u8,
    filter_method: u8,
    interlace_method: u8,
    pixels_per_unit_x: u32,
    pixels_per_unit_y: u32,
    pixels_per_unit_specifier: PngUnit,
    pixels: Vec<Vec<Pixel>>,
}

impl PngImage {
    pub fn create_image() -> PngImage {
        PngImage {
            width: 0,
            height: 0,
            color_type: 0,
            bit_depth: 0,
            png_channels: Vec::new(),
            palette: Vec::new(),
            compression_method: 0,
            filter_method: 0,
            interlace_method: 0,
            pixels_per_unit_x: 0,
            pixels_per_unit_y: 0,
            pixels_per_unit_specifier: PngUnit::UNKN,
            pixels: Vec::new()
        }
    }

    pub fn set_width(&mut self, width: u32) {
        self.width = width;
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn set_height(&mut self, height: u32) {
        self.height = height;
    }

    pub fn get_height(&self) -> u32 {
        self.height
    }

    pub fn set_color_type(&mut self, color_type: u8) {
        self.color_type = color_type;
    }

    pub fn get_color_type(&self) -> u8 {
        self.color_type
    }

    pub fn set_bit_depth(&mut self, bit_depth: u8) {
        self.bit_depth = bit_depth;
    }

    pub fn get_bit_depth(&self) -> u8 {
        self.bit_depth
    }

    pub fn push_png_channel(&mut self, png_channel: PngChannel) {
        self.png_channels.push(png_channel);
    }

    pub fn get_png_channels(&self) -> &Vec<PngChannel> {
        &self.png_channels
    }

    pub fn push_palette_entry(&mut self, entry: Color) {
        self.palette.push(entry);
    }

    pub fn get_palette_entry(&self, index: usize) -> &Color {
        self.palette.get(index).expect("Attempted to retrieve invalid palette entry!")
    }

    pub fn set_compression_method(&mut self, compression_method: u8) {
        self.compression_method = compression_method;
    }

    pub fn get_compression_method(&self) -> u8 {
        self.compression_method
    }

    pub fn set_filter_method(&mut self, filter_method: u8) {
        self.filter_method = filter_method;
    }

    pub fn get_filter_method(&self) -> u8 {
        self.filter_method
    }

    pub fn set_interlace_method(&mut self, interlace_method: u8) {
        self.interlace_method = interlace_method;
    }

    pub fn get_interlace_method(&self) -> u8 {
        self.interlace_method
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

    pub fn set_pixels(&mut self, pixels: Vec<Vec<Pixel>>) {
        self.pixels = pixels;
    }

    pub fn pixels(&self) -> &Vec<Vec<Pixel>> {
        &self.pixels
    }

    pub fn move_pixels(self) -> Vec<Vec<Pixel>> {
        self.pixels
    }
}
use crate::image_analyzers::png::{PngImage, PngUnit};

pub mod png;

#[derive(Debug)]
pub enum ImageUnit {
    METER,
    INCH,
    UNKNOWN
}

#[derive(Debug)]
pub struct Pixel {
    red: u8,
    blue: u8,
    green: u8,
    alpha: u8
}

#[derive(Debug)]
pub struct Image {
    width: u32,
    height: u32,
    width_in_units: f64,
    height_in_units: f64,
    unit_specifier: ImageUnit,
    pixels: Vec<Vec<Pixel>>
}

impl From<PngImage> for Image {
    fn from(png: PngImage) -> Self {
        Image {
            width: png.get_width().clone(),
            height: png.get_height().clone(),
            width_in_units: png.get_width().clone() as f64 / png.get_pixels_per_unit_x().clone() as f64,
            height_in_units: png.get_height().clone() as f64 / png.get_pixels_per_unit_y().clone() as f64,
            unit_specifier: match png.get_pixels_per_unit_specifier() {
                PngUnit::METER => ImageUnit::METER,
                _ => ImageUnit::UNKNOWN
            },
            pixels: Vec::new()
        }
    }
}
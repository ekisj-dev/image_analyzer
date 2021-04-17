use crate::image_analyzers::png::{PngImage, PngUnit};
use std::fmt::{Display, Formatter};
use std::fmt;

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
    green: u8,
    blue: u8,
    alpha: u8
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Pixel {
        Pixel {
            red,
            green,
            blue,
            alpha
        }
    }

    pub fn red(&self) -> &u8 {
        &self.red
    }

    pub fn green(&self) -> &u8 {
        &self.green
    }

    pub fn blue(&self) -> &u8 {
        &self.blue
    }

    pub fn alpha(&self) -> &u8 {
        &self.alpha
    }
}

pub struct Image {
    width: u32,
    height: u32,
    width_in_units: f64,
    height_in_units: f64,
    unit_specifier: ImageUnit,
    pixels: Vec<Vec<Pixel>>
}

impl Display for Image {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Image: Width={}, Height={}, Width in Units={} {:?}, Height in Units={} {:?}",
               self.width,
               self.height,
               self.width_in_units, self.unit_specifier,
               self.height_in_units, self.unit_specifier)
    }
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
            pixels: png.move_pixels()
        }
    }
}
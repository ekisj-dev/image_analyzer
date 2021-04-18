use crate::types::color::Color;
use std::fmt::{Display, Formatter};
use std::fmt;

#[derive(Debug)]
pub enum ImageUnit {
    METER,
    INCH,
    UNKNOWN
}

#[derive(Debug, Clone)]
pub struct Pixel {
    color: Color,
    alpha: u8
}

impl Display for Pixel {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{:3} {:3} {:3} {:3}", self.alpha, self.red(), self.green(), self.blue())
    }
}

impl From<Pixel> for u32 {
    fn from(pixel: Pixel) -> Self {
        let (a, r, g, b) = (pixel.alpha as u32, pixel.red() as u32, pixel.green() as u32, pixel.blue() as u32);

        (a << 24) | (r << 16) | (g << 8) | b
    }
}

impl Pixel {
    pub fn new(red: u8, green: u8, blue: u8, alpha: u8) -> Pixel {
        Pixel {
            color: Color::new(red, green, blue),
            alpha
        }
    }

    pub fn empty() -> Pixel {
        Pixel {
            color: Color::new(0, 0, 0),
            alpha: 0
        }
    }

    pub fn red(&self) -> u8 {
        self.color.red()
    }

    pub fn green(&self) -> u8 {
        self.color.green()
    }

    pub fn blue(&self) -> u8 {
        self.color.blue()
    }

    pub fn alpha(&self) -> u8 {
        self.alpha
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

impl Image {
    pub fn new(width: u32, height: u32, width_in_units: f64, height_in_units: f64, unit_specifier: ImageUnit, pixels: Vec<Vec<Pixel>>) -> Self {
        Image { width, height, width_in_units, height_in_units, unit_specifier, pixels }
    }
}

impl Image {
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn width_in_units(&self) -> f64 {
        self.width_in_units
    }
    pub fn height_in_units(&self) -> f64 {
        self.height_in_units
    }
    pub fn unit_specifier(&self) -> &ImageUnit {
        &self.unit_specifier
    }
    pub fn pixels(&self) -> &Vec<Vec<Pixel>> {
        &self.pixels
    }
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
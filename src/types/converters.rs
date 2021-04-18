use crate::image_analyzers::png::{PngImage, PngUnit};
use crate::types::image::{Image, ImageUnit};

impl From<PngImage> for Image {
    fn from(png: PngImage) -> Self {
        Image::new(png.get_width(),
                   png.get_height(),
                   png.get_width() as f64 / png.get_pixels_per_unit_x() as f64,
                   png.get_height() as f64 / png.get_pixels_per_unit_y() as f64,
                   match png.get_pixels_per_unit_specifier() {
                       PngUnit::METER => ImageUnit::METER,
                       _ => ImageUnit::UNKNOWN
                   },
                   png.move_pixels())
    }
}
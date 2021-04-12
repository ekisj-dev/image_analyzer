use std::str::FromStr;
use std::fmt::{Display, Formatter};
use std::fmt;
use std::ffi::OsStr;

#[derive(Debug)]
pub enum ImageType {
    PNG
}

#[derive(Debug, Clone)]
pub struct UnknownTypeError;

impl Display for UnknownTypeError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "Unknown type override provided. Possible values include: {:?}", vec![ImageType::PNG])
    }
}

impl FromStr for ImageType {
    type Err = UnknownTypeError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "PNG" | "png" => Ok(ImageType::PNG),
            _ => Err(UnknownTypeError),
        }
    }
}

pub fn identify_image_by_bytes(image_bytes: &Vec<u8>) -> Option<ImageType> {
    if identify_png_by_bytes(image_bytes) {
        Some(ImageType::PNG)
    } else {
        None
    }
}

pub fn identify_image_by_name(image_extension: Option<&OsStr>) -> Option<ImageType> {
    match image_extension?.to_str()? {
        "png" | "PNG" => {
            Some(ImageType::PNG)
        },
        _ => None,
    }
}

fn identify_png_by_bytes(image_bytes: &Vec<u8>) -> bool {
    const PNG_HEADER_LEN:usize = 8;
    if image_bytes.len() < PNG_HEADER_LEN {
        return false;
    }
    let first_eight_bytes = &image_bytes[..PNG_HEADER_LEN];

    first_eight_bytes.eq(vec![137, 80, 78, 71, 13, 10, 26, 10].as_slice())
}

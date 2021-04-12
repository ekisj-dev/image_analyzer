pub enum ImageType {
    PNG
}

pub fn identify_image(image_bytes: &Vec<u8>) -> Option<ImageType> {
    if identify_png(image_bytes) {
        Some(ImageType::PNG)
    } else {
        None
    }
}

fn identify_png(image_bytes: &Vec<u8>) -> bool {
    const PNG_HEADER_LEN:usize = 8;
    if image_bytes.len() < PNG_HEADER_LEN {
        return false;
    }
    let first_eight_bytes = &image_bytes[..PNG_HEADER_LEN];

    first_eight_bytes.eq(vec![137, 80, 78, 71, 13, 10, 26, 10].as_slice())
}

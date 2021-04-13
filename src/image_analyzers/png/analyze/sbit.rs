use crate::image_analyzers::png::chunk::PngChunk;
use crate::image_analyzers::png::{PngImage, PngChannel};

pub fn analyze_sbit_chunk(sbit_chunk: &PngChunk, png_image: &mut PngImage) {
    println!("Analyzing sBIT chunk...");
    let chunk_data = sbit_chunk.get_data();

    let color_type = png_image.get_color_type();

    match color_type {
        0 => {
            println!("Grayscale color type detected...");
            if chunk_data.len() != 1 {
                println!("sBIT chunk incorrectly defined, PNG may be corrupted.");
                return;
            }

            let num_sig_bits = chunk_data.get(0);
            let gray_channel = match num_sig_bits {
                Some(num) => PngChannel::GRAY(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the grayscale channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            png_image.push_png_channel(gray_channel);
        },
        2 => {
            println!("Truecolor color type detected...");
            if chunk_data.len() != 3 {
                println!("sBIT chunk incorrectly defined, PNG may be corrupted.");
                return;
            }

            let red_sig_bits = chunk_data.get(0);
            let green_sig_bits = chunk_data.get(1);
            let blue_sig_bits = chunk_data.get(2);

            let red_channel = match red_sig_bits {
                Some(num) => PngChannel::RED(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the red channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            let green_channel = match green_sig_bits {
                Some(num) => PngChannel::GREEN(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the green channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            let blue_channel = match blue_sig_bits {
                Some(num) => PngChannel::BLUE(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the blue channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            // Only push channels if the chunk was not corrupt.
            png_image.push_png_channel(red_channel);
            png_image.push_png_channel(green_channel);
            png_image.push_png_channel(blue_channel);
        },
        3 => {
            println!("Indexed color type detected...");
            if chunk_data.len() != 3 {
                println!("sBIT chunk incorrectly defined, PNG may be corrupted.");
                return;
            }

            let red_sig_bits = chunk_data.get(0);
            let green_sig_bits = chunk_data.get(1);
            let blue_sig_bits = chunk_data.get(2);

            let red_channel = match red_sig_bits {
                Some(num) => PngChannel::RED(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the red channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            let green_channel = match green_sig_bits {
                Some(num) => PngChannel::GREEN(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the green channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            let blue_channel = match blue_sig_bits {
                Some(num) => PngChannel::BLUE(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the blue channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            // Only push channels if the chunk was not corrupt.
            png_image.push_png_channel(red_channel);
            png_image.push_png_channel(green_channel);
            png_image.push_png_channel(blue_channel);
        },
        4 => {
            println!("Greyscale with alpha channel type detected...");
            if chunk_data.len() != 2 {
                println!("sBIT chunk incorrectly defined, PNG may be corrupted.");
                return;
            }

            let num_sig_bits = chunk_data.get(0);
            let alpha_sig_bits = chunk_data.get(1);

            let gray_channel = match num_sig_bits {
                Some(num) => PngChannel::GRAY(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the grayscale channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            let alpha_channel = match alpha_sig_bits {
                Some(num) => PngChannel::ALPHA(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the alpha channel.\
                     Chunk may be corrupt");
                    return;
                }
            };

            png_image.push_png_channel(gray_channel);
            png_image.push_png_channel(alpha_channel);
        },
        6 => {
            println!("Truecolor with Alpha Channel color type detected...");
            if chunk_data.len() != 4 {
                println!("sBIT chunk incorrectly defined, PNG may be corrupted.");
                return;
            }

            let red_sig_bits = chunk_data.get(0);
            let green_sig_bits = chunk_data.get(1);
            let blue_sig_bits = chunk_data.get(2);
            let alpha_sig_bits = chunk_data.get(3);

            let red_channel = match red_sig_bits {
                Some(num) => PngChannel::RED(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the red channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            let green_channel = match green_sig_bits {
                Some(num) => PngChannel::GREEN(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the green channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            let blue_channel = match blue_sig_bits {
                Some(num) => PngChannel::BLUE(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the blue channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            let alpha_channel = match alpha_sig_bits {
                Some(num) => PngChannel::ALPHA(num.clone()),
                None => {
                    println!("Could not find significant bit definition for the alpha channel.\
                     Chunk may be corrupt.");
                    return;
                }
            };

            png_image.push_png_channel(red_channel);
            png_image.push_png_channel(green_channel);
            png_image.push_png_channel(blue_channel);
            png_image.push_png_channel(alpha_channel);

        },
        _ => println!("Unknown color type, unable to analyze sBIT chunk.")
    }

    println!("Finished analyzing sBIT chunk!");
}
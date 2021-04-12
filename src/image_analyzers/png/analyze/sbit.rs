use crate::image_analyzers::png::chunk::PngChunk;
use crate::image_analyzers::png::PngImage;

pub fn analyze_sbit_chunk(sbit_chunk: &PngChunk, png_image: &mut PngImage) {
    println!("Analyzing sBIT chunk...");
    let chunk_data = sbit_chunk.get_data();

    let color_type = png_image.get_color_type();

    match color_type {
        0 => {
            println!("Grayscale color type detected...");
            if chunk_data.len() != 1 {
                println!("sBIT chunk incorrectly defined, PNG may be corrupted.");
            } else {
                let num_sig_bits = chunk_data.get(0);
                match num_sig_bits {
                    Some(num) => println!("{} significant bits in the source data.", num),
                    None => println!("Could not find significant bit definition.\
                     Chunk may be corrupt.")
                }
            }
        },
        2 => (),
        3 => (),
        4 => (),
        6 => {
            println!("Truecolor with Alpha Channel color type detected...");
            if chunk_data.len() != 4 {
                println!("sBIT chunk incorrectly defined, PNG may be corrupted.");
            } else {
                let red_sig_bits = chunk_data.get(0);
                let green_sig_bits = chunk_data.get(1);
                let blue_sig_bits = chunk_data.get(2);
                let alpha_sig_bits = chunk_data.get(3);

                match red_sig_bits {
                    Some(num) => println!("{} significant bits in the source data for the red channel.", num),
                    None => println!("Could not find significant bit definition for the red channel. Chunk may be corrupt.")
                }

                match green_sig_bits {
                    Some(num) => println!("{} significant bits in the source data for the green channel.", num),
                    None => println!("Could not find significant bit definition for the green channel. Chunk may be corrupt.")
                }

                match blue_sig_bits {
                    Some(num) => println!("{} significant bits in the source data for the blue channel.", num),
                    None => println!("Could not find significant bit definition for the blue channel. Chunk may be corrupt.")
                }

                match alpha_sig_bits {
                    Some(num) => println!("{} significant bits in the source data for the alpha channel.", num),
                    None => println!("Could not find significant bit definition for the alpha channel. Chunk may be corrupt.")
                }
            }
        },
        _ => println!("Unknown color type, unable to analyze sBIT chunk.")
    }

    println!("Finished analyzing sBIT chunk!");
}
use std::env;
use std::fs;

use image_analyzer::image_identifiers;
use image_analyzer::image_identifiers::ImageType;
use image_analyzer::image_analyzers::png::parse::collect_chunks;
use image_analyzer::image_analyzers::png::analyze::analyze_chunk;
use image_analyzer::image_analyzers::png::PngImage;

const PRETTY_PRINT_WIDTH: u8 = 16;

fn help() {
    println!("image_analyzer usage: ");
    println!("\t-f <file name>");
    println!("\t\tThis argument defines the file that the Image Analyzer tool is reading.");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("Welcome to the Image Analyzer!");

    if args.len() < 2 {
        help();
        std::process::exit(-1);
    }

    let file_name = &args[1];

    let contents = fs::read(file_name)
        .expect("Something went wrong while reading the given file name.");


    let mut pretty_index: u8 = 0;
    for byte in &contents {
        if pretty_index >= PRETTY_PRINT_WIDTH {
            print!("\n");
            pretty_index = 0;
        }

        print!("{} ", byte);
        pretty_index += 1;
    }

    println!();

    println!("File bytes printed!");
    println!("Printed out {} bytes.", contents.len());

    let image_type = image_identifiers::identify_image(&contents)
        .expect("Unable to identify image!");

    match image_type {
        ImageType::PNG => {
            println!("Found a PNG!");

            let chunks = collect_chunks(contents)
                .expect("Chunks were unable to be collected from PNG file.");
            // `contents` unusable after this point

            println!("Found {} chunks.", chunks.len());

            let mut png_image: PngImage = PngImage::create_image();

            for chunk in chunks {
                println!("Found a chunk with data length of {}, with a type of {:?}",
                         chunk.get_length(),
                         chunk.get_type());

                analyze_chunk(&chunk, &mut png_image);
            }
        }
    }
}

use std::fs;

use structopt::StructOpt;

use image_analyzer::image_identifiers;
use image_analyzer::image_identifiers::ImageType;
use image_analyzer::image_analyzers::png::parse::collect_chunks;
use image_analyzer::image_analyzers::png::analyze::analyze_chunk;
use image_analyzer::image_analyzers::png::PngImage;

const PRETTY_PRINT_WIDTH: u8 = 16;

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    #[structopt(short = "f", long = "file", help = "Path to the image that will this program will attempt to analyze.")]
    file_name: std::path::PathBuf,

    #[structopt(short = "t", long = "type", help = "Overrides the extension to perform a specific analysis.")]
    file_type: Option<ImageType>,
}

fn main() {
    println!("Welcome to the Image Analyzer!");
    let opts: Opt = Opt::from_args();

    let file_name = &opts.file_name;

    file_name.extension().expect("File did not have an extension");

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

    let image_type = &opts.file_type
        .or_else(|| image_identifiers::identify_image_by_name(file_name.extension()))
        .or_else (|| image_identifiers::identify_image_by_bytes(&contents))
        .expect("Unable to determine file type!");

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

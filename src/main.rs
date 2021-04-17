use std::fs;

use structopt::StructOpt;
use simple_logger::SimpleLogger;
use log::LevelFilter;
use log::{info, debug, trace};

use image_analyzer::image_identifiers;
use image_analyzer::image_identifiers::ImageType;
use image_analyzer::image_analyzers::png::parse::collect_chunks;
use image_analyzer::image_analyzers::png::analyze::analyze_chunk;
use image_analyzer::image_analyzers::png::PngImage;
use image_analyzer::image_analyzers::Image;

const PRETTY_PRINT_WIDTH: u8 = 16;

#[derive(StructOpt)]
struct Opt {
    #[structopt(parse(from_os_str))]
    #[structopt(short = "f", long = "file", help = "Path to the image that will this program will attempt to analyze.")]
    file_name: std::path::PathBuf,

    #[structopt(short = "t", long = "type", help = "Overrides the extension to perform a specific analysis.")]
    file_type: Option<ImageType>,

    #[structopt(short, long, parse(from_occurrences))]
    verbosity: u8,

    #[structopt(short, long)]
    quiet: bool,
}

fn main() {
    let opts: Opt = Opt::from_args();
    let log_level;
    if opts.quiet {
        log_level = LevelFilter::Off;
    } else {
        log_level = match opts.verbosity {
            0 => LevelFilter::Info,
            1 => LevelFilter::Debug,
            _ => LevelFilter::Trace
        };
    }


    SimpleLogger::new().with_level(log_level).init().unwrap();

    let file_name = &opts.file_name;

    file_name.extension().expect("File did not have an extension");

    let contents = fs::read(file_name)
        .expect("Something went wrong while reading the given file name.");


    if log_level >= LevelFilter::Trace {
        let mut byte_string: String = String::new();
        byte_string.push('\n');

        let mut pretty_index: u8 = 0;
        for byte in &contents {
            if pretty_index >= PRETTY_PRINT_WIDTH {
                byte_string.push('\n');
                pretty_index = 0;
            }

            byte_string.push_str(format!("{} ", byte).as_str());
            pretty_index += 1;
        }

        trace!("Image Bytes: {}", byte_string);
        trace!("Printed out {} bytes.", contents.len());
    }

    let image_type = &opts.file_type
        .or_else(|| image_identifiers::identify_image_by_name(file_name.extension()))
        .or_else (|| image_identifiers::identify_image_by_bytes(&contents))
        .expect("Unable to determine file type!");

    let final_image: Image;

    match image_type {
        ImageType::PNG => {
            debug!("Found a PNG!");

            let chunks = collect_chunks(contents)
                .expect("Chunks were unable to be collected from PNG file.");
            // `contents` unusable after this point

            trace!("Found {} chunks.", chunks.len());

            let mut png_image: PngImage = PngImage::create_image();

            for chunk in chunks {
                trace!("Found a chunk with data length of {}, with a type of {:?}",
                         chunk.get_length(),
                         chunk.get_type());

                analyze_chunk(&chunk, &mut png_image);
            }

            final_image = Image::from(png_image);
        }
    }

    info!("Final image data: {}", final_image);
}

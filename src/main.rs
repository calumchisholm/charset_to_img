#![warn(clippy::all, clippy::pedantic)]

extern crate image;

use std::fs;
use std::io::Read;

use structopt::StructOpt;

use image::imageops::{self, FilterType};

#[derive(StructOpt)]
#[structopt(
    name = "charset_to_img",
    about = "Generates a bitmap file from a file containing 8x8 raster characters."
)]
struct Args {
    #[structopt(help = "Input file of 8x8 character bitmaps")]
    input_file: String,

    #[structopt(
        help = "Output image file. File format is determined based on the file extension (png or gif only)"
    )]
    output_file: String,

    #[structopt(
        short = "c",
        long = "columns",
        help = "Number of characters to display per row",
        default_value = "16"
    )]
    columns: u16,

    #[structopt(short = "m", long = "mirror", help = "Mirror the character symbols")]
    mirror: bool,

    #[structopt(
        short = "h",
        long = "scale-horizontal",
        help = "Horizontal scale factor to apply to output file",
        default_value = "2"
    )]
    scale_h: u32,

    #[structopt(
        short = "v",
        long = "scale-vertical",
        help = "Vertical scale factor to apply to output file",
        default_value = "2"
    )]
    scale_v: u32,
}

fn main() {
    const CHAR_WIDTH: u16 = 8;
    const CHAR_HEIGHT: u16 = 8;

    let args = Args::from_args();

    match fs::File::open(args.input_file) {
        Err(error) => {
            eprintln!("Problem opening the file: {error:?}");
        }
        Ok(mut file) => {
            let mut contents: Vec<u8> = Vec::new();

            match file.read_to_end(&mut contents) {
                Err(error) => {
                    eprintln!("Problem reading the file: {error:?}");
                }
                Ok(length) => {
                    #[allow(clippy::cast_possible_truncation)]
                    let file_size = length as u32;
                    let columns: u16 = args.columns;
                    let width: u32 = (columns * CHAR_WIDTH).into();

                    let rem: u32 = file_size % width;
                    let height: u32 = if rem > 0 {
                        // Character count doesn't divide evenly by the number of columns.
                        ((file_size - rem) / u32::from(columns)) + u32::from(CHAR_HEIGHT)
                    } else {
                        file_size / u32::from(columns)
                    };

                    let mut imgbuf = image::ImageBuffer::from_pixel(
                        width,
                        height,
                        image::Rgb([0xFF, 0x00, 0xFF]),
                    );

                    let mut char_row: u16 = 0;
                    let mut char_col: u16 = 0;
                    let mut char_line: u16 = 0;

                    for character in contents.iter().take(length) {
                        let mut bit: u8 = if args.mirror { 0x01 } else { 0x80 };
                        let y: u32 = ((char_row * CHAR_HEIGHT) + char_line).into();
                        for p in 0..CHAR_WIDTH {
                            let x: u32 = ((char_col * CHAR_WIDTH) + p).into();
                            let colour: u8 = if (character & bit) > 0 { 0xFF } else { 0 };

                            imgbuf.put_pixel(x, y, image::Rgb([colour, colour, colour]));

                            if args.mirror {
                                bit <<= 1;
                            } else {
                                bit >>= 1;
                            };
                        }

                        char_line += 1;
                        if char_line == CHAR_HEIGHT {
                            char_line = 0;
                            char_col += 1;

                            if char_col == columns {
                                char_col = 0;
                                char_row += 1;
                            }
                        }
                    }

                    imageops::resize(
                        &imgbuf,
                        args.scale_h * width,
                        args.scale_v * height,
                        FilterType::Nearest,
                    )
                    .save(args.output_file)
                    .unwrap();

                    println!("Done");
                }
            }
        }
    }
}
# charset_to_img

A simple Rust program that generates a bitmap file from a file containing 8x8 raster characters.

## Description

This program converts a binary file containing 8x8 character bitmaps into a visual image representation. It's particularly useful for visualizing character sets or font data stored in a binary format.

## Features

- Converts 8x8 raster characters to a bitmap image
- Configurable number of columns for character layout
- Option to mirror character symbols
- Adjustable horizontal and vertical scaling of the output image
- Supports PNG and GIF output formats

## Usage

```
charset_to_img [OPTIONS] <input_file> <output_file>
```

### Arguments

- `<input_file>`: Input file of 8x8 character bitmaps
- `<output_file>`: Output image file (PNG or GIF). Output format determined based on file extension.

### Options

- `-c, --columns <columns>`: Number of characters to display per row [default: 16]
- `-m, --mirror`: Mirror the character symbols
- `-h, --scale-horizontal <scale_h>`: Horizontal scale factor [default: 2]
- `-v, --scale-vertical <scale_v>`: Vertical scale factor [default: 2]

## Building and Running

1. Ensure you have [Rust and Cargo](https://rustup.rs/#) installed
2. Clone this repository
3. Run `cargo build --release` to compile the program
4. Execute the program with appropriate arguments

### Examples

8 column output, no scaling
```
charset_to_img character_rom.bin character_rom_8col.gif --columns 8
```

16 column output, scaled 4x horizontally and vertically and mirrored.
```
charset_to_img character_rom.bin character_rom_16col_quad_mirror.png --columns 16 --scale-horizontal 4 --scale-vertical 4 --mirror
```

32 column output, double-height.
```
charset_to_img character_rom.bin character_rom_32col_doubleheight.png --columns 32 --scale-horizontal 1 --scale-vertical 2
```

## Dependencies

- `structopt`: For parsing command-line arguments
- `image`: For image processing and output

## License

This project is licensed under the MIT License.

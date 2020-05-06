extern crate imguni;

#[macro_use]
extern crate clap;
extern crate font_rs;
extern crate image;
extern crate rand;

use clap::{App, Arg};
use font_rs::font;
use std::fs::File;
use std::io::Read;
use std::vec::Vec;

use imguni::{image_to_unicode, GlyphsOrder, GlyphsRandom};

fn main() {
    let matches = App::new("imguni")
        .about("Image to unicode character converter")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("image")
                .help("The path to the image to convert")
                .value_name("FILE")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("glyph")
                .long("glyph")
                .short("g")
                .help("The glyph to use ")
                .value_name("GLYPH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .long("out")
                .short("o")
                .help("The image output path")
                .value_name("OUT")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("font")
                .long("font")
                .short("f")
                .help("The font to use")
                .value_name("FONT")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("tile")
                .long("tile")
                .short("t")
                .help("The tile size")
                .value_name("TILE")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("no-random")
                .long("no-random")
                .help("Use the glyphs random")
                .required(false),
        )
        .get_matches();
    
    // parse command line argument
    let image_in = matches.value_of("image").unwrap();
    let font_file = matches
        .value_of("font")
        .unwrap_or("assets/NotoMono-Regular.ttf");
    let image_out = matches.value_of("out").unwrap_or("out.png");
    let glyphs = matches.value_of("glyph").unwrap_or("@");
    let tile: usize = matches.value_of("tile").unwrap_or("15").parse().unwrap();
    let no_random = matches.is_present("no-random");

    // load font file
    let mut f = File::open(&font_file).unwrap();
    let mut data = Vec::new();
    let font = match f.read_to_end(&mut data) {
        Err(e) => panic!("failed to read {}, {}", font_file, e),
        Ok(_) => match font::parse(&data) {
            Ok(font) => font,
            Err(_) => panic!("failed to parse {}", font_file),
        },
    };

    // load image file
    let img = match image::open(image_in) {
        Ok(img) => img,
        Err(e) => panic!("Err: {}", e),
    };

    let res = if no_random {
        image_to_unicode(img, tile, font, GlyphsOrder::new(glyphs))
    } else {
        image_to_unicode(img, tile, font, GlyphsRandom::new(glyphs))
    };
    match res {
        Ok(img) => img.save(image_out).unwrap(),
        Err(e) => panic!("Error converting image: {}", e),
    }
}

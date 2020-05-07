extern crate imguni;

extern crate rusttype;
#[macro_use]
extern crate clap;
extern crate font_loader;
extern crate image;
extern crate rand;

use clap::{App, Arg};
use font_loader::system_fonts;
use rusttype::Font;
use std::fs::File;
use std::io::Read;
use std::path::Path;
use std::vec::Vec;

use imguni::{image_to_unicode, GlyphsOrder, GlyphsRandom};

fn main() {
    let matches = App::new("imguni")
        .about("Image to unicode character converter")
        .version(crate_version!())
        .author(crate_authors!())
        .arg(
            Arg::with_name("image")
                .help("The path of the input image")
                .value_name("FILE")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::with_name("glyph")
                .long("glyph")
                .short("g")
                .help("The glyphs to use")
                .value_name("GLYPH")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("out")
                .long("out")
                .short("o")
                .help("The image output name")
                .value_name("OUT")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("font")
                .long("font")
                .short("f")
                .help("The font to use, either a name or a file (.ttf, .oft)")
                .value_name("FONT")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("tile")
                .long("tile")
                .short("t")
                .help("The glyph size")
                .value_name("TILE")
                .takes_value(true)
                .required(false),
        )
        .arg(
            Arg::with_name("no-random")
                .long("no-random")
                .help("Disable the random selection of the glyphs")
                .required(false),
        )
        .get_matches();

    // parse command line argument
    let image_in = matches.value_of("image").unwrap();
    let image_out = matches.value_of("out").unwrap_or("out.png");
    let glyphs = matches.value_of("glyph").unwrap_or("@");
    let tile: u32 = matches.value_of("tile").unwrap_or("15").parse().unwrap();
    let no_random = matches.is_present("no-random");

    // load font file
    let font_data = if matches.is_present("font") {
        let font_arg = matches.value_of("font").unwrap();
        if Path::new(font_arg).exists() {
            println!("Use font file '{}'", font_arg);
            let mut f = File::open(font_arg).unwrap();
            let mut data = Vec::new();
            f.read_to_end(&mut data).unwrap();
            data
        } else {
            let mut property = system_fonts::FontPropertyBuilder::new()
                .family(font_arg)
                .build();
            if system_fonts::query_specific(&mut property).is_empty() {
                panic!("System font '{}' not founded", font_arg)
            }
            println!("Use system font '{}'", font_arg);
            let (font_data, _) = system_fonts::get(&property).unwrap();
            font_data
        }
    } else {
        println!("Use system monospace font");
        let property = system_fonts::FontPropertyBuilder::new().monospace().build();
        let (font, _) = system_fonts::get(&property).unwrap();
        font
    };

    // load font
    let font = match Font::try_from_bytes(&font_data) {
        Some(font) => font,
        None => panic!("Failed to load font"),
    };

    // load image file
    let img = match image::open(image_in) {
        Ok(img) => img,
        Err(e) => panic!("Err: {}", e),
    };

    // convert the image
    let res = if no_random {
        image_to_unicode(img, tile, font, GlyphsOrder::new(glyphs))
    } else {
        image_to_unicode(img, tile, font, GlyphsRandom::new(glyphs))
    };
    match res {
        Ok(img) => img.save(image_out).unwrap(),
        Err(e) => panic!("Err: {}", e),
    }
}

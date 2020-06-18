extern crate halftoneglyph;

extern crate font_kit;
#[macro_use]
extern crate clap;
extern crate image;
extern crate rand;

use clap::{App, Arg};
use font_kit::family_name::FamilyName;
use font_kit::font::Font;
use font_kit::properties::Properties;
use font_kit::source::SystemSource;
use halftoneglyph::{image_to_unicode, GlyphsOrder, GlyphsRandom};
use std::io;
use std::io::Write;
use std::path::Path;

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

    // load font
    let font = if matches.is_present("font") {
        let font_arg = matches.value_of("font").unwrap();
        if Path::new(font_arg).exists() {
            println!("Use font file '{}'", font_arg);
            Font::from_path(font_arg, 0)
        } else {
            match SystemSource::new().select_family_by_name(font_arg) {
                Ok(h) => {
                    println!("Use system font '{}'", font_arg);
                    h.fonts()[0].load()
                }
                Err(e) => panic!("System font '{}': {}", font_arg, e),
            }
        }
    } else {
        println!("Use system monospace font");
        SystemSource::new()
            .select_best_match(&[FamilyName::Monospace], &Properties::new())
            .unwrap()
            .load()
    };
    let font = font.expect("Cannot laod font");

    start(&format!("Load image '{}'", image_in));
    let img = match image::open(image_in) {
        Ok(img) => img,
        Err(e) => panic!("Err: {}", e),
    };
    done();

    // convert the image
    start("Converting");
    let res = if no_random {
        image_to_unicode(img, tile, font, GlyphsOrder::new(glyphs))
    } else {
        image_to_unicode(img, tile, font, GlyphsRandom::new(glyphs))
    };
    match res {
        Ok(img) => img.save(image_out).unwrap(),
        Err(e) => panic!("Err: {}", e),
    };
    done();
    println!("Immage saved to '{}'", image_out);
}

fn start(msg: &str) {
    print!("{}... ", msg);
    io::stdout().flush().unwrap();
}

fn done() {
    println!("done");
}

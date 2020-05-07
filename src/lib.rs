extern crate image;
extern crate rand;
extern crate rusttype;

use image::imageops::resize;
use image::DynamicImage;
use image::{imageops::FilterType, GrayImage, Luma};
use rand::prelude::*;
use rusttype::{point, Font, Scale};

pub trait GlyphsIter {
    fn next(&mut self) -> char;
}

pub struct GlyphsRandom {
    data: Vec<char>,
    rng: ThreadRng,
}
impl GlyphsRandom {
    pub fn new(glyphs: &str) -> Self {
        GlyphsRandom {
            data: glyphs.chars().collect(),
            rng: rand::thread_rng(),
        }
    }
}
impl GlyphsIter for GlyphsRandom {
    fn next(&mut self) -> char {
        unsafe {
            *self
                .data
                .get_unchecked(self.rng.gen_range(0, self.data.len()))
        }
    }
}

pub struct GlyphsOrder {
    data: Vec<char>,
    i: usize,
}
impl GlyphsOrder {
    pub fn new(glyphs: &str) -> Self {
        GlyphsOrder {
            data: glyphs.chars().collect(),
            i: 0,
        }
    }
}
impl GlyphsIter for GlyphsOrder {
    fn next(&mut self) -> char {
        let g = unsafe { *self.data.get_unchecked(self.i) };
        self.i = (self.i + 1) % self.data.len();
        g
    }
}

pub fn image_to_unicode(
    img: DynamicImage,
    tile: u32,
    font: Font,
    mut glyphs: impl GlyphsIter,
) -> Result<GrayImage, String> {
    let img = img.grayscale().into_luma();
    let origin_width = img.width();
    let origin_height = img.height();

    let new_with = (origin_width as f32 / tile as f32).floor() as u32;
    let new_height = (origin_height as f32 / tile as f32).floor() as u32;
    let img = resize(
        &img,
        new_with as u32,
        new_height as u32,
        FilterType::Lanczos3,
    );

    let img_width = new_with * tile;
    let img_height = new_height * tile;

    println!("{}x{}", new_with, new_height);
    let mut image = DynamicImage::new_luma8(img_width, img_height).to_luma();
    for (j, r) in img.rows().enumerate() {
        for (i, p) in r.enumerate() {
            // TODO: check that the glyph is correctly rendered
            let g = font.glyph(glyphs.next());
            let g = g.scaled(Scale::uniform((p.0[0] as u32 * tile / 255) as f32));
            let g = g.positioned(point(0.0, 0.0));
            if let Some(bounding_box) = g.pixel_bounding_box() {
                g.draw(|x, y, v| {
                    let cx = (tile - bounding_box.width() as u32) / 2;
                    let cy = (tile - bounding_box.height() as u32) / 2;
                    image.put_pixel(
                        (i as u32 * tile) + x + cx,
                        (j as u32 * tile) + y + cy,
                        Luma([(v * 255.0) as u8]),
                    );
                });
            }
        }
    }
    Ok(image)
}

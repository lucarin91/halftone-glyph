extern crate font_rs;
extern crate image;
extern crate rand;

use font_rs::font::Font;
use image::imageops::resize;
use image::DynamicImage;
use image::{imageops::FilterType, GrayImage};
use rand::prelude::*;

pub trait GlyphsIter {
    fn next(&mut self, font: &Font) -> Option<u16>;
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
    fn next(&mut self, font: &Font) -> Option<u16> {
        unsafe {
            font.lookup_glyph_id(
                *self
                    .data
                    .get_unchecked(self.rng.gen_range(0, self.data.len())) as u32,
            )
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
    fn next(&mut self, font: &Font) -> Option<u16> {
        let g = unsafe { *self.data.get_unchecked(self.i) };
        self.i = (self.i + 1) % self.data.len();
        font.lookup_glyph_id(g as u32)
    }
}

pub fn image_to_unicode(
    img: DynamicImage,
    tile: usize,
    font: Font,
    mut glyphs: impl GlyphsIter,
) -> Result<GrayImage, String> {
    let img = img.grayscale().into_luma();
    let origin_width = img.width();
    let origin_height = img.height();

    let new_with = (origin_width as f32 / tile as f32).floor() as u32;
    let new_height = (origin_height as f32 / tile as f32).floor() as u32;
    let img = resize(&img, new_with, new_height, FilterType::Lanczos3);

    let img_width = new_with as usize * tile;
    let img_height = new_height as usize * tile;
    let mut img_vec = vec![0; img_width * img_height];
    for (j, r) in img.rows().enumerate() {
        for (i, p) in r.enumerate() {
            if let Some(glyph_id) = glyphs.next(&font) {
                match font.render_glyph(glyph_id, p.0[0] as u32 * tile as u32 / 255) {
                    Some(glyph) => {
                        for jj in 0..glyph.height {
                            for ii in 0..glyph.width {
                                img_vec
                                    [(img_width * j * tile + (img_width * jj)) + (i * tile + ii)] =
                                    glyph.data[glyph.width * jj + ii];
                            }
                        }
                    }
                    None => return Err(format!("failed to render {}", glyph_id)),
                }
            } else {
                return Err("glyph not found".to_owned());
            }
        }
    }
    match GrayImage::from_vec(img_width as u32, img_height as u32, img_vec) {
        Some(img) => Ok(img),
        None => Err("cannot create immage".to_owned()),
    }
}

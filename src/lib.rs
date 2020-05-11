extern crate font_kit;
extern crate image;
extern crate pathfinder_geometry;
extern crate rand;

use font_kit::canvas::{Canvas, Format, RasterizationOptions};
use font_kit::error::GlyphLoadingError;
use font_kit::font::Font;
use font_kit::hinting::HintingOptions;
use image::imageops::resize;
use image::DynamicImage;
use image::{imageops::FilterType, GrayImage, Luma};
use pathfinder_geometry::transform2d::Transform2F;
use pathfinder_geometry::vector::Vector2I;
use rand::prelude::*;
use std::fmt::Display;
use std::string::String;

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

pub struct ImmageConvertError(String);

impl Display for ImmageConvertError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
        write!(f, "{}", self.0)
    }
}

impl From<GlyphLoadingError> for ImmageConvertError {
    fn from(e: GlyphLoadingError) -> Self {
        ImmageConvertError(e.to_string())
    }
}

pub fn image_to_unicode(
    img: DynamicImage,
    tile: u32,
    font: Font,
    mut glyphs: impl GlyphsIter,
) -> Result<GrayImage, ImmageConvertError> {
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
    let mut image = DynamicImage::new_luma8(img_width, img_height).to_luma();

    for (i, j, p) in img.enumerate_pixels() {
        match font.glyph_for_char(glyphs.next()) {
            Some(glyph_id) => {
                let mut canvas = Canvas::new(Vector2I::splat(tile as i32), Format::A8);
                let size = (p.0[0] as u32 * tile / 255) as f32;

                let raster_rect = font.raster_bounds(
                    glyph_id,
                    size,
                    Transform2F::default(),
                    HintingOptions::None,
                    RasterizationOptions::GrayscaleAa,
                )?;
                font.rasterize_glyph(
                    &mut canvas,
                    glyph_id,
                    size,
                    Transform2F::from_translation(-raster_rect.origin().to_f32()),
                    HintingOptions::None,
                    RasterizationOptions::GrayscaleAa,
                )?;

                for (ii, p) in canvas.pixels.into_iter().enumerate() {
                    image.put_pixel(
                        (i as u32 * tile) + (ii as u32 % canvas.size.x() as u32),
                        (j as u32 * tile) + (ii as u32 / canvas.size.x() as u32),
                        Luma([p as u8]),
                    );
                }
            }
            None => return Err(ImmageConvertError("Cannot load Glyph".to_string())),
        }
    }
    Ok(image)
}

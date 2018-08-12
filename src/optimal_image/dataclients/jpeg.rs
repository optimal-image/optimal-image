extern crate dssim;
extern crate image;
extern crate imgref;
extern crate lodepng;
extern crate rgb;

use dataclients::{ImageDataResult, Loader};
use image::*;
use rgb::RGBA;
use std::path::Path;

pub struct Jpeg {
    pub name: String,
}

impl Loader<ImageError> for Jpeg {
    fn load<P: AsRef<Path>>(&self, path: P) -> ImageDataResult<ImageError> {
        let image = image::open(path)?;
        let (width, height) = image.dimensions();
        let buffer = image
            .to_rgba()
            .pixels()
            .map(|pixel| {
                let [r, g, b, a] = pixel.to_rgba().data;
                let rgba: RGBA<f32>;
                // convert u8 (0/255) color values to (0-1) f32 ranges
                rgba = RGBA {
                    r: r as f32 / 255.0,
                    g: g as f32 / 255.0,
                    b: b as f32 / 255.0,
                    a: a as f32 / 255.0,
                };
                rgba
            })
            .collect();

        Ok(imgref::Img::new(
            buffer,
            width as usize,
            height as usize,
        ))
    }
}

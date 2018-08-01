extern crate dssim;
extern crate image;
extern crate imgref;
extern crate lodepng;

use dataclients::ImageSpec;
use dssim::*;
use image::{open, DynamicImage, ImageResult};
use imgref::ImgVec;
use std::env;
use std::io;
use std::path::Path;

pub struct Png {
    pub name: String,
}

impl Png {
    pub fn load<P: AsRef<Path>>(&self, path: P) -> Result<ImgVec<dssim::RGBAPLU>, lodepng::Error> {
        let image = lodepng::decode32_file(path.as_ref())?;
        Ok(imgref::Img::new(
            image.buffer.to_rgbaplu(),
            image.width,
            image.height,
        ))
    }
}

// impl ImageSpec for Png {
//     fn new(&self, name: String) -> Png {
//         Png{ name }
//     }
// }

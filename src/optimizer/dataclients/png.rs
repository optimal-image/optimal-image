extern crate image;
extern crate dssim;
extern crate imgref;
extern crate lodepng;

use std::env;
use std::io;
use std::path::{Path};
use image::{open, ImageResult, DynamicImage};
use imgref::{ImgVec};
use dssim::*;
use dataclients::ImageSpec;

#[derive(Debug)]
pub struct Png {
    name: String
}

impl ImageSpec for Png {
    fn new(&self, name: String) -> Png {
        Png{ name }
    }

    fn load<P: AsRef<Path>>(&self, path: P) -> Result<ImgVec<dssim::RGBAPLU>, io::Error> {
        let image = lodepng::decode32_file(path.as_ref())?;
        Ok(imgref::Img::new(image.buffer.to_rgbaplu(), image.width, image.height))
    }
}
extern crate dssim;
extern crate image;
extern crate imgref;
extern crate lodepng;

use dataclients::{ImageDataResult, Loader};
use dssim::*;
use image::*;
use std::path::Path;

pub struct Png {
    pub name: String,
}

impl Png {
    pub fn new() -> Png {
        Png {
            name: String::from("png-dataclient"),
        }
    }
}

impl Loader for Png {
    fn load<P: AsRef<Path>>(&self, path: P) -> ImageDataResult<ImageError> {
        let image = match lodepng::decode32_file(path.as_ref()) {
            Ok(img) => img,
            _ => {
                return Err(ImageError::FormatError(String::from(
                    "cannot decode png",
                )))
            }
        };
        Ok(imgref::Img::new(
            image.buffer.to_rgbaplu(),
            image.width,
            image.height,
        ))
    }
}

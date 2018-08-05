extern crate dssim;
extern crate image;
extern crate imgref;
extern crate lodepng;

use dataclients::{ImageDataResult, Loader};
use dssim::*;
use std::path::Path;

pub struct Png {
    pub name: String,
}

impl Loader<lodepng::Error> for Png {
    fn load<P: AsRef<Path>>(&self, path: P) -> ImageDataResult<lodepng::Error> {
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

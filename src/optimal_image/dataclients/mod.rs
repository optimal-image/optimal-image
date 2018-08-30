extern crate dssim;
extern crate rgb;
use image::ImageError;
use imgref::ImgVec;
use rgb::*;
use std::error::Error;
use std::path::Path;
mod jpeg;
mod png;

pub type ImageData = ImgVec<RGBA<f32>>;
pub type ImageDataResult<E> = Result<ImageData, E>;

pub trait Loader {
    fn load<P: AsRef<Path>>(&self, path: P) -> ImageDataResult<ImageError>;
}

pub use self::jpeg::Jpeg;
pub use self::png::Png;

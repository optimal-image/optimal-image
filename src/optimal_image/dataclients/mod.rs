extern crate dssim;
extern crate rgb;
use imgref::ImgVec;
use rgb::*;
use std::error::Error;
use std::path::Path;
mod jpeg;
mod png;

pub type ImageData = ImgVec<RGBA<f32>>;
pub type ImageDataResult<E> = Result<ImageData, E>;

pub trait Loader<E: Error> {
    fn load<P: AsRef<Path>>(&self, path: P) -> ImageDataResult<E>;
}

pub use self::jpeg::Jpeg;
pub use self::png::Png;

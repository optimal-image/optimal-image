extern crate dssim;
use std::path::Path;
use std::error::Error;
use imgref::ImgVec;
mod png;

pub type ImageData = ImgVec<dssim::RGBAPLU>;
pub type ImageDataResult<E = Error> = Result<ImageData, E>;

pub trait Loader<E: Error> {
    fn load<P: AsRef<Path>>(&self, path: P) -> ImageDataResult<E>;
}

pub use self::png::Png;

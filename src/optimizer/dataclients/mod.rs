use std::io;
use std::path::{Path};
use imgref::{ImgVec};
use dssim::*;
mod png;

trait ImageSpec {
    fn new(&self, name: String) -> Self where Self: Sized;
    fn load<T: AsRef<Path>>(&self, path: T) -> Result<ImgVec<dssim::RGBAPLU>, io::Error>;
}

pub use self::png::Png;
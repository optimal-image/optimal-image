use dataclients::ImageData;
use std::error::Error;

mod image_jpeg;

pub trait FromImageData<T> {
    fn from_image_data(img: &ImageData) -> Result<T, Box<Error>>;
}

pub trait ToImageData {
    fn to_image_data(&self) -> Result<ImageData, Box<Error>>;
}

pub use self::image_jpeg::ImageJpegImage;


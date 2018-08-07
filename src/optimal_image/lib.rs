extern crate dssim;
extern crate image;
extern crate imgref;
extern crate lodepng;
extern crate rgb;

use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub enum ImageFormat {
    JPEG,
    PNG,
    WEBP,
    JPEGXR,
}

#[derive(Debug, Clone)]
pub struct EncodingConfig {
    format: ImageFormat,
    quality: u8,
}

impl PartialOrd for EncodingConfig {
    fn partial_cmp(&self, other: &EncodingConfig) -> Option<Ordering> {
        Some(self.quality.cmp(&other.quality))
    }
}

impl PartialEq for EncodingConfig {
    fn eq(&self, other: &EncodingConfig) -> bool {
        self.quality == other.quality
    }
}

#[derive(Debug, Clone)]
pub struct ImageConfig {
    id: String,
    encoding_config: EncodingConfig,
}

impl PartialOrd for ImageConfig {
    fn partial_cmp(&self, other: &ImageConfig) -> Option<Ordering> {
        self.encoding_config.partial_cmp(&other.encoding_config)
    }
}

impl PartialEq for ImageConfig {
    fn eq(&self, other: &ImageConfig) -> bool {
        self.encoding_config == other.encoding_config
    }
}

#[derive(Debug, Clone)]
pub struct ImageScore {
    config: ImageConfig,
    score: f64,
    size: u64,
}

pub mod dataclients;
pub mod search;

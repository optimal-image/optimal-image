extern crate rgb;
extern crate imgref;
extern crate libwebp_rust;
use dataclients::ImageData;
use encoders::{Encode, FromImageData, ToImageData};
use std::error::Error;
use std::fmt;
use std::os::raw;

pub struct WebpImage {
    pub data: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl FromImageData<WebpImage> for WebpImage {
    fn from_image_data(img: &ImageData) -> Result<WebpImage, Box<Error>> {
        let byte_data: Vec<u8> =
            img.pixels().fold(Vec::new(), |mut data: Vec<u8>, pixel| {
                let rgb::RGBA { r, g, b, a } = pixel;
                data.push((r * 255.0) as u8);
                data.push((g * 255.0) as u8);
                data.push((b * 255.0) as u8);
                data.push((a * 255.0) as u8);
                data
            });

        Ok(WebpImage {
            data: byte_data,
            width: img.width() as usize,
            height: img.height() as usize,
        })
    }
}

#[derive(Debug)]
struct WebpError;

impl fmt::Display for WebpError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[webp]Error")
    }
}

impl Error for WebpError {
    fn description(&self) -> &str {
        "[webp] Error"
    }
}

impl ToImageData for WebpImage {
    fn to_image_data(&self) -> Result<ImageData, Box<Error>> {
        let img = &self.data;
        let buffer: Result<Vec<rgb::RGBA<f32>>, WebpError> = img.chunks(4).map(|data| {
            if let [r, g, b, a] = *data {
                let rgba: rgb::RGBA<f32>;
                // convert u8 (0/255) color values to (0-1) f32 ranges
                rgba = rgb::RGBA {
                    r: r as f32 / 255.0,
                    g: g as f32 / 255.0,
                    b: b as f32 / 255.0,
                    a: a as f32 / 255.0,
                };
                Ok(rgba)
            } else {
                Err(WebpError)
            }
        }).collect();

        Ok(imgref::Img::new(
            buffer.expect("error while converting RGBA chunks to rgb::RGBA pixels"),
            self.width,
            self.height,
        ))
    }
}

impl Encode<WebpImage> for WebpImage {
    fn encode(&self, quality: u8) -> Result<WebpImage, Box<Error>> {
        let buffer: *mut *mut u8 = 0 as *mut *mut u8;
        unsafe {
            libwebp_rust::WebPEncodeRGBA(
                &self.data[0] as *const u8,
                self.width as raw::c_int,
                self.height as raw::c_int,
                4,
                quality as f32,
                buffer,
            );
        }
        unimplemented!();
    }
}

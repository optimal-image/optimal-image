extern crate image;
extern crate imgref;
extern crate rgb;
use dataclients::ImageData;
use encoders::{FromImageData, ToImageData, Encode};
use std::error::Error;
use image::jpeg;
use image::{ImageDecoder, ConvertBuffer};

pub struct ImageJpegImage {
    pub img: image::RgbaImage,
}

impl FromImageData<ImageJpegImage> for ImageJpegImage {
    fn from_image_data(img: &ImageData) -> Result<ImageJpegImage, Box<Error>> {
        let byte_data: Vec<u8> =
            img.pixels().fold(Vec::new(), |mut data: Vec<u8>, pixel| {
                let rgb::RGBA { r, g, b, a } = pixel;
                data.push((r * 255.0) as u8);
                data.push((g * 255.0) as u8);
                data.push((b * 255.0) as u8);
                data.push((a * 255.0) as u8);
                data
            });

        let img = image::ImageBuffer::from_raw(
            img.width() as u32,
            img.height() as u32,
            byte_data,
        ).unwrap();

        Ok(ImageJpegImage { img: img })
    }
}

impl ToImageData for ImageJpegImage {
    fn to_image_data(&self) -> Result<ImageData, Box<Error>> {
        let img = &self.img;
        let buffer = img
            .pixels()
            .map(|pixel| {
                let [r, g, b, a] = pixel.data;
                let rgba: rgb::RGBA<f32>;
                // convert u8 (0/255) color values to (0-1) f32 ranges
                rgba = rgb::RGBA {
                    r: r as f32 / 255.0,
                    g: g as f32 / 255.0,
                    b: b as f32 / 255.0,
                    a: a as f32 / 255.0,
                };
                rgba
            })
            .collect();

        Ok(imgref::Img::new(
            buffer,
            img.width() as usize,
            img.height() as usize,
        ))
    }
}

impl Encode<ImageJpegImage> for ImageJpegImage {
    fn encode(&self, quality: u8) -> Result<ImageJpegImage, Box<Error>> {
        let mut buffer: Vec<u8> = Vec::new();
        let img = &self.img;
        {
            let mut encoder = jpeg::JPEGEncoder::new_with_quality(&mut buffer, quality);
            let _ = encoder.encode(&img.clone().into_raw(), img.width(), img.height(), image::ColorType::RGBA(8));
        }

        let mut decoder = jpeg::JPEGDecoder::new(buffer.as_slice());
        let decoded = match decoder.read_image().unwrap() {
            image::DecodingResult::U8(vec) => vec,
            _ => panic!()
        };
        let output_img: image::RgbImage = image::ImageBuffer::from_raw(img.width() as u32, img.height() as u32, decoded).unwrap();
        Ok(ImageJpegImage{ img: output_img.convert() })
    }
}

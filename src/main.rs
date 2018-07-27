extern crate image;
extern crate dssim;
extern crate imgref;
extern crate lodepng;
use std::env;
use std::path::{Path};
use image::{open, ImageResult, DynamicImage};
use imgref::{ImgVec};
use dssim::*;

fn loadJpeg(url: &Path ) -> ImageResult<DynamicImage> {
    open(url)
}

fn loadPng<P: AsRef<Path>>(path: P) -> Result<ImgVec<dssim::RGBAPLU>, lodepng::Error> {
    let image = lodepng::decode32_file(path.as_ref())?;
    Ok(imgref::Img::new(image.buffer.to_rgbaplu(), image.width, image.height))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let (program, rest_args) = args.split_at(1);
    let a = &rest_args[0];
    let b = &rest_args[1];

    let mut context = dssim::new();
    let imageA = loadPng(Path::new(a)).unwrap();
    let imageB = loadPng(Path::new(b)).unwrap();
    let dssimImageA = context.create_image(&imageA).unwrap();
    let dssimImageB = context.create_image(&imageA).unwrap();

    let (val, _) = context.compare(&dssimImageA, dssimImageB);

    println!("{:?}", val);
}

extern crate dssim;
extern crate optimal_image;

use optimal_image::compress::*;
use optimal_image::dataclients::*;
use optimal_image::search::*;
use optimal_image::ImageFormat;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (_program, rest_args) = args.split_at(1);
    let a = &rest_args[0];
    let b = &rest_args[1];

    let png_client = Png::new();
    let jpg_client = Jpeg::new();
    let mut context = dssim::new();
    // let image_a = png_client.load(Path::new(a)).unwrap();
    // let image_b = png_client.load(Path::new(b)).unwrap();
    // let dssim_image_a = context.create_image(&image_a).unwrap();
    // let dssim_image_b = context.create_image(&image_b).unwrap();

    let image_a = jpg_client.load(Path::new(a)).unwrap();
    let image_b = jpg_client.load(Path::new(b)).unwrap();
    let dssim_image_a = context.create_image(&image_a).unwrap();
    let dssim_image_b = context.create_image(&image_b).unwrap();

    // let mut search = Search::from_path(
    //     Path::new(a),
    //     SearchOptions {
    //         threshold: 0.004,
    //         quality_range: (70, 91),
    //         formats: vec![ImageFormat::JPEG, ImageFormat::PNG],
    //     },
    // ).unwrap();
    // search.run();
    // println!("{:?}", search.get_result());

    // let (val, _) = context.compare(&dssim_image_a, dssim_image_b);

    // println!("{:?}", val);

    let img = VipsImage::from_file(Path::new(a).to_str().unwrap()).unwrap();
    let img2 = VipsImage::from_file(Path::new(b).to_str().unwrap()).unwrap();
    // let img2 = VipsImage::from_image_data(&image_a).unwrap();

    img2.to_image_data();

    unsafe { assert_eq!((*img.img).Xsize, (*img2.img).Xsize) };
    unsafe { assert_eq!((*img.img).Ysize, (*img2.img).Ysize) };
    unsafe { assert_eq!((*img.img).Bands, (*img2.img).Bands) };
    unsafe { assert_eq!((*img.img).Length, (*img2.img).Length) };
    unsafe { assert_eq!((*img.img).Compression, (*img2.img).Compression) };

    println!("{:?}", img.img);
}

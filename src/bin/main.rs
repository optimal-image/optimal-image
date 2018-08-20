extern crate dssim;
extern crate optimal_image;

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

    let png_client = Png {
        name: String::from("png-dataclient"),
    };

    let jpg_client = Jpeg {
        name: String::from("jpeg-dataclient"),
    };

    let mut context = dssim::new();
    // let image_a = png_client.load(Path::new(a)).unwrap();
    // let image_b = png_client.load(Path::new(b)).unwrap();
    // let dssim_image_a = context.create_image(&image_a).unwrap();
    // let dssim_image_b = context.create_image(&image_b).unwrap();

    let image_a = jpg_client.load(Path::new(a)).unwrap();
    let image_b = jpg_client.load(Path::new(b)).unwrap();
    let dssim_image_a = context.create_image(&image_a).unwrap();
    let dssim_image_b = context.create_image(&image_b).unwrap();

    let mut search = Search::from_path(
        Path::new(a),
        SearchOptions {
            threshold: 0.004,
            quality_range: (70, 91),
            formats: vec![ImageFormat::JPEG, ImageFormat::PNG],
        },
    ).unwrap();
    search.run();
    println!("{:?}", search.get_result());

    let (val, _) = context.compare(&dssim_image_a, dssim_image_b);

    println!("{:?}", val);
}

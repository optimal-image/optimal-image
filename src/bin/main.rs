extern crate dssim;
extern crate optimal_image;

use std::env;
use std::path::{Path};
use optimal_image::dataclients::Png;
use dssim::*;

fn main() {
    let args: Vec<String> = env::args().collect();
    let (_program, rest_args) = args.split_at(1);
    let a = &rest_args[0];
    let b = &rest_args[1];

    let png_client = Png { name: "png-dataclient".to_string() };

    let mut context = dssim::new();
    let image_a = png_client.load(Path::new(a)).unwrap();
    let image_b = png_client.load(Path::new(b)).unwrap();
    let dssim_image_a = context.create_image(&image_a).unwrap();
    let dssim_image_b = context.create_image(&image_b).unwrap();

    let (val, _) = context.compare(&dssim_image_a, dssim_image_b);

    println!("{:?}", val);
}

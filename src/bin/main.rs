extern crate dssim;
extern crate optimal_image;

use optimal_image::search::*;
use optimal_image::ImageFormat;
use std::env;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let image_path = &args.get(1).unwrap();
    let mut search = Search::from_path(
        Path::new(image_path),
        SearchOptions {
            threshold: 0.0005,
            quality_range: (70, 91),
            formats: vec![ImageFormat::JPEG],
        },
    ).unwrap();
    search.run();
    println!("{:?}", search.get_result());
}

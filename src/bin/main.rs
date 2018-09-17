extern crate clap;
extern crate dssim;
extern crate optimal_image;

use clap::{App, Arg, SubCommand};
use optimal_image::dataclients::{Jpeg, Loader};
use optimal_image::search::*;
use optimal_image::ImageFormat;
use std::path::Path;

fn main() {
    let matches = App::new("Optimal Image")
        .subcommand(SubCommand::with_name("search")
            .version("0.2.0")
            .about("Determine optimal compression settings for an image")
            .arg(Arg::with_name("original").index(1).required(true))
            .arg(Arg::with_name("threshold")
                 .short("t")
                 .long("threshold")
                 .help("threshold difference from original (lower is better)")
                 .required(true)
                 .takes_value(true),
            )
            .arg(Arg::with_name("range")
                 .short("r")
                 .long("range")
                 .help("Compression range to search (e.g. --range min max)")
                 .number_of_values(2)
                 .value_names(&["MIN", "MAX"])
                 .required(true),
            ))
        .subcommand(SubCommand::with_name("diff")
            .about("Calculate DSSIM difference between 2 images")
            .arg(Arg::with_name("original").index(1).required(true))
            .arg(Arg::with_name("variant").index(2).required(true)),
        )
        .get_matches();

    // DSSIM Diff
    if let Some(matches) = matches.subcommand_matches("diff") {
        let mut context = dssim::new();
        let jpg_client = Jpeg::new();
        let original_path = matches.value_of("original").unwrap();
        let original = jpg_client
            .load(Path::new(original_path))
            .expect("cannot open original image");
        let dssim_original = context
            .create_image(&original)
            .expect("cannot create original DSSIM image");
        let variant_path = matches.value_of("variant").unwrap();
        let variant = jpg_client
            .load(Path::new(variant_path))
            .expect("cannot open variant image");
        let dssim_variant = context
            .create_image(&variant)
            .expect("cannot create variant DSSIM image");

        let (val, _) = context.compare(&dssim_original, dssim_variant);

        println!("DSSIM value: {}", val);
        return;
    }

    // Optimal image search
    if let Some(matches) = matches.subcommand_matches("search") {
        let mut range = matches.values_of("range").unwrap();
        let min: u8 = range
            .next()
            .unwrap()
            .parse()
            .expect("min range is not a valid integer");
        let max: u8 = range
            .next()
            .unwrap()
            .parse()
            .expect("max range is not a valid integer");
        let threshold: f64 = matches
            .value_of("threshold")
            .unwrap()
            .parse()
            .expect("threshold is not a valid number");
        let original_path = matches.value_of("original").unwrap();

        let mut search = Search::from_path(
            Path::new(original_path),
            SearchOptions {
                threshold: threshold,
                quality_range: (min, max),
                formats: vec![ImageFormat::JPEG],
            },
        ).unwrap();
        search.run();

        println!("Optimal Config: {:?}", search.get_result());
        return
    }

    // Show usage by default
    println!("{}", matches.usage());
    println!("{}", matches.subcommand_matches("search").unwrap().usage());
}

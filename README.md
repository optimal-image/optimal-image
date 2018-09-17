# optimal-image
[![Build Status](https://travis-ci.org/optimal-image/optimal-image.svg?branch=master)](https://travis-ci.com/optimal-image/optimal-image)


## Development
- For the first time you want to run, you can run the `make init` to intialize the `githooks`

- Once you have `cargo` available locally, build the package as
```bash
cargo run
```

## Usage
```
Optimal Image 0.2.0
Determine optimal compression settings for an image
USAGE:
    optimal_image <original> --range <MIN> <MAX> --threshold <threshold> [SUBCOMMAND]
FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
OPTIONS:
    -r, --range <MIN> <MAX>        Compression range to search (e.g. --range min max)
    -t, --threshold <threshold>    threshold difference from original (lower is better)
ARGS:
    <original>
SUBCOMMANDS:
    diff    Calculate DSSIM difference between 2 images
    help    Prints this message or the help of the given subcommand(s)
```

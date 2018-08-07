# optimal-image
[![Build Status](https://travis-ci.com/axyz/optimal-image.svg?branch=master)](https://travis-ci.com/axyz/optimal-image)


## Development
- For the first time you want to run, you can run the `make init` to intialize the `githooks`

- Once you have `cargo` available locally, build the package as
```bash
cargo run
```
- Now you can verify the tool  by comparing 2 images (source http://www.gaeawiki.com/) as below:
```
./target/optimal_image data/Iceland_scenery.png data/Iceland_scenery-min.png
```

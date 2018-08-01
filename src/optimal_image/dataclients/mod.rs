use std::fmt;
use imgref::{ImgVec};
use std::error::Error;
use std::path::{Path};
use dssim::*;
mod png;

pub trait ImageSpec {
}

pub use self::png::Png;
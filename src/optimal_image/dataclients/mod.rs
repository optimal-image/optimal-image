use dssim::*;
use imgref::ImgVec;
use std::error::Error;
use std::fmt;
use std::path::Path;
mod png;

pub trait ImageSpec {}

pub use self::png::Png;

extern crate vips_sys as ffi;

mod image;
mod instance;

pub use image::VipsImage;
pub use instance::VipsInstance;
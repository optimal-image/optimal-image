use vips;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null;

pub struct VipsInstance {}

impl VipsInstance {
    pub fn new(name: &str) -> Result<VipsInstance, Box<Error>> {
        let c = CString::new(name)?;
        unsafe {
            vips::vips_init(c.as_ptr());
        }
        Ok(VipsInstance {})
    }
}

impl Drop for VipsInstance {
    fn drop(&mut self) {
        unsafe {
            vips::vips_shutdown();
        }
    }
}

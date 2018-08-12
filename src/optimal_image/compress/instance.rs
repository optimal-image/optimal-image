use std::ffi::{CString, CStr};
use std::error::Error;
use std::os::raw::{c_int, c_void, c_char};
use std::ptr::null;
use std::marker::PhantomData;

pub struct VipsInstance { }

impl VipsInstance {
    pub fn new(name:&str) -> Result<VipsInstance, Box<Error>> {
        let c = CString::new(name)?;
        unsafe {
            ffi::vips_init(c.as_ptr());
        }
        Ok(VipsInstance {})
    }
}

impl Drop for VipsInstance {
    fn drop(&mut self) {
        unsafe {
            ffi::vips_shutdown();
        }
    }
}


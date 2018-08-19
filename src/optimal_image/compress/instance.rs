use std::error::Error;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null;
use vips;

/// `VipsInstance` implements the initialisation and cleanup of vips instance
/// by calling `vips_init()` and `vips_shutdown()`
pub struct VipsInstance {}

impl VipsInstance {
    /// This function starts up `libvips`
    /// See http://jcupitt.github.io/libvips/API/current/libvips-vips.html#vips-init
    pub fn new(name: &str) -> Result<VipsInstance, Box<Error>> {
        let c = CString::new(name)?;
        unsafe {
            vips::vips_init(c.as_ptr());
        }
        Ok(VipsInstance {})
    }
}

impl Drop for VipsInstance {
    /// This function drops caches and closes plugins
    /// See http://jcupitt.github.io/libvips/API/current/libvips-vips.html#vips-shutdown
    fn drop(&mut self) {
        unsafe {
            vips::vips_shutdown();
        }
    }
}

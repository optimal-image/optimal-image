
use std::ffi::{CString, CStr};
use std::error::Error;
use std::sync::atomic::AtomicBool;
use std::os::raw::{c_int, c_void, c_char};
use std::ptr::null;
use std::marker::PhantomData;
use std::sync::atomic::Ordering::Relaxed;


pub struct VipsImage<'a> {
    pub c: *mut ffi::VipsImage,
    marker: PhantomData<&'a()>,
}

impl<'a> Drop for VipsImage<'a> {
    fn drop(&mut self) {
        unsafe {
            ffi::g_object_unref(self.c as *mut c_void);
        }
    }
}

impl<'a> VipsImage<'a> {
    pub fn new() -> Result<VipsImage<'a>, Box<Error>> {
        let c = unsafe { ffi::vips_image_new() };
        result(c)
    }

    pub fn from_file<S: Into<Vec<u8>>>(path: S) -> Result<VipsImage<'a>, Box<Error>> {
        let path = CString::new(path)?;
        let c = unsafe { ffi::vips_image_new_from_file(path.as_ptr(), null() as *const c_char) };
        result(c)
    }

    pub fn write_to_file<S: Into<Vec<u8>>>(&self, path: S) -> Result<(), Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_image_write_to_file(self.c as *mut ffi::VipsImage, path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }
    
    pub fn jpegsave<S: Into<Vec<u8>>>(&mut self, path: S) -> Result<(), Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe { ffi::vips_jpegsave(self.c as *mut ffi::VipsImage, path.as_ptr(), null() as *const c_char) };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }
}

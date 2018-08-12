use vips;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;

const NULL_LIST: *const c_char = null() as *const c_char;

/// VipsImage implements the port of VipsImage class from vips
///
pub struct VipsImage<'a> {
    /// `img` represents the `VipsImage` class mentioned here
    /// http://jcupitt.github.io/libvips/API/current/VipsImage.html
    pub img: *mut vips::VipsImage,
    /// `marker`for indicating the lifetime of img as `a`
    marker: PhantomData<&'a ()>,
}

impl<'a> Drop for VipsImage<'a> {
    fn drop(&mut self) {
        unsafe {
            vips::g_object_unref(self.img as *mut c_void);
        }
    }
}

impl<'a> VipsImage<'a> {
    /// `from_file()` creates an instance of `VipsImage` from a given file path
    pub fn from_file<S: Into<Vec<u8>>>(
        path: S,
    ) -> Result<VipsImage<'a>, Box<Error>> {
        let path = CString::new(path)?;
        let c =
            unsafe { vips::vips_image_new_from_file(path.as_ptr(), NULL_LIST) };
        result(c)
    }

    /// `write_to_file()` writes an instance of `VipsImage` to a given file path
    pub fn write_to_file<S: Into<Vec<u8>>>(
        &self,
        path: S,
    ) -> Result<(), Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe {
            vips::vips_image_write_to_file(
                self.img as *mut vips::VipsImage,
                path.as_ptr(),
                NULL_LIST,
            )
        };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }

    /// `jpegsave()` saves a `jpeg` image with a given `quality` factor
    pub fn jpegsave<S: Into<Vec<u8>>>(
        &mut self,
        path: S,
        quality: i32,
    ) -> Result<(), Box<Error>> {
        let path = CString::new(path)?;
        let ret = unsafe {
            vips::vips_jpegsave(
                self.img as *mut vips::VipsImage,
                path.as_ptr(),
                "Q\0".as_ptr(),
                quality as i32,
                NULL_LIST,
            )
        };
        match ret {
            0 => Ok(()),
            _ => Err(current_error().into()),
        }
    }
}


pub fn current_error() -> String {
    let msg = unsafe {
        CStr::from_ptr(vips::vips_error_buffer())
    };
    msg.to_str().unwrap().to_string()
}

fn result<'a>(ptr: *mut vips::VipsImage) -> Result<VipsImage<'a>, Box<Error>> {
    if ptr.is_null() {
        Err(current_error().into())
    } else {
        Ok(VipsImage { img: ptr, marker: PhantomData })
    }
}


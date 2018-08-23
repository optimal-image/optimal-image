extern crate image;
extern crate imgref;
extern crate rgb;
use dataclients::ImageData;
use std::error::Error;
use std::ffi::{CStr, CString};
use std::marker::PhantomData;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr::null;
use std::slice;
use std::mem;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use vips;

const NULL_LIST: *const c_char = null() as *const c_char;

/// VipsImage implements the port of VipsImage class from vips
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
        let c = unsafe {
            let mut out: *mut *mut vips::VipsImage =
                vips::vips_image_new() as *mut *mut vips::VipsImage;
            let rgb_image =
                vips::vips_image_new_from_file(path.as_ptr(), NULL_LIST);
            // new_from_file can return RGB image, we have to add alpha in order
            // to be sure to have an RGBA image
            vips::vips_addalpha(rgb_image, out);
            // as we are using a native VipsImage we need to manually deref it
            vips::g_object_unref(rgb_image as *mut c_void);
            *(out)
        };
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

    pub fn from_buffer(
        buf: *const c_void,
        len: usize,
    ) -> Result<VipsImage<'a>, Box<Error>> {
        let c =
            unsafe { vips::vips_image_new_from_buffer(buf, len, NULL_LIST) };
        result(c)
    }

    pub fn from_memory(
        data: *const c_void,
        size: usize,
        width: i32,
        height: i32,
        bands: i32,
    ) -> Result<VipsImage<'a>, Box<Error>> {
        let c = unsafe {
            vips::vips_image_new_from_memory(
                data,
                size,
                width,
                height,
                bands,
                vips::VipsBandFormat::VIPS_FORMAT_UCHAR,
            )
        };
        result(c)
    }

    pub fn from_image_data(
        img: &ImageData,
    ) -> Result<VipsImage<'a>, Box<Error>> {
        let byte_data: Vec<u8> =
            img.pixels().fold(Vec::new(), |mut data: Vec<u8>, pixel| {
                let rgb::RGBA { r, g, b, a } = pixel;
                data.push(r as u8);
                data.push(g as u8);
                data.push(b as u8);
                data.push(a as u8);
                data
            });
        let byte_data_ptr: *const c_void =
            &byte_data as *const _ as *const c_void;
        let bands: i32 = 4; // RGBA (4 bands)
        let size = img.buf.len() * bands as usize;
        let width = img.width() as i32;
        let height = img.height() as i32;

        let c =
            VipsImage::from_memory(byte_data_ptr, size, width, height, bands)?;
        result(c.img)
    }

    pub fn to_image_data(&self) -> Result<ImageData, Box<Error>> {
        let vector: Vec<u8> = unsafe {
            let mut result_size: usize = 0;
            let memory: *mut u8 = vips::vips_image_write_to_memory(
                self.img,
                &mut result_size as *mut usize,
            ) as *mut u8;
            let slice = slice::from_raw_parts_mut(memory, result_size);
            let vec = slice.to_vec();
            // FIXME: we should free the memory, but it is segfaulting
            // vips::g_object_unref(memory as *mut c_void);
            vec
        };

        // let buffer: ImageData = vector.chunks(4).map(|chunk| {
        //     chunk.iter().fold(RGBA {}, |pixel, byte| {
        //         b
        //     })
        // }).collect();


        let width = unsafe { (*self.img).Xsize };
        let height = unsafe { (*self.img).Ysize };

        // unsafe { println!("{:?}", buffer) };

        unimplemented!();

        // Ok(imgref::Img::new(buffer, width as usize, height as usize))
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
    let msg = unsafe { CStr::from_ptr(vips::vips_error_buffer()) };
    msg.to_str().unwrap().to_string()
}

fn result<'a>(ptr: *mut vips::VipsImage) -> Result<VipsImage<'a>, Box<Error>> {
    if ptr.is_null() {
        Err(current_error().into())
    } else {
        Ok(VipsImage {
            img: ptr,
            marker: PhantomData,
        })
    }
}

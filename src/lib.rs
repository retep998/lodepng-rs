// Copyright © 2014, Peter Atashian

extern crate image;
extern crate libc;

use image::{
    GenericImage,
    ImageBuf,
    Rgba,
};
use libc::{
    c_uchar,
    c_uint,
    c_void,
    free,
    size_t,
};
use std::c_vec::CVec;
use std::io::fs::File;
use std::mem::zeroed;

#[link(name = "lodepng")]
extern {
    fn lodepng_decode32(
        outbuf: *mut *mut c_uchar,
        width: *mut c_uint,
        height: *mut c_uint,
        inbuf: *const c_uchar,
        insize: size_t,
    ) -> c_uint;
    fn lodepng_encode32(
        outbuf: *mut *mut c_uchar,
        outsize: *mut size_t,
        inbuf: *const c_uchar,
        width: c_uint,
        height: c_uint,
    ) -> c_uint;
}

pub fn load(path: &Path) -> Result<ImageBuf<Rgba<u8>>, &'static str> {
    let mut file = File::open(path);
    let data = match file.read_to_end() {
        Ok(data) => data,
        Err(_) => return Err("Failed to read file"),
    };
    unsafe {
        let mut width = zeroed();
        let mut height = zeroed();
        let mut outbuf = zeroed();
        match lodepng_decode32(
            &mut outbuf, &mut width, &mut height,
            data.as_ptr(), data.len() as size_t,
        ) {
            0 => (),
            _ => return Err("Failed to decode png data"),
        }
        let pixels = CVec::new(
            outbuf as *mut Rgba<u8>,
            (width * height) as uint,
        ).as_slice().to_vec();
        free(outbuf as *mut c_void);
        Ok(ImageBuf::from_pixels(pixels, width, height))
    }
}
pub fn save(img: &ImageBuf<Rgba<u8>>, path: &Path) -> Result<(), &'static str> {
    let pixels = unsafe {
        let mut outbuf = zeroed();
        let mut outsize = zeroed();
        let (width, height) = img.dimensions();
        match lodepng_encode32(
            &mut outbuf, &mut outsize,
            img.pixelbuf().as_ptr() as *const c_uchar,
            width, height,
        ) {
            0 => (),
            _ => return Err("Failed to encode png data"),
        }
        let pixels = CVec::new(
            outbuf as *mut u8,
            outsize as uint,
        ).as_slice().to_vec();
        free(outbuf as *mut c_void);
        pixels
    };
    let mut file = File::create(path);
    match file.write(pixels.as_slice()) {
        Ok(_) => Ok(()),
        Err(_) => Err("Failed to write to file"),
    }
}

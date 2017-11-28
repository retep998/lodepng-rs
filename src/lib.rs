// Copyright © 2017, Peter Atashian

extern crate image;
extern crate libc;

use image::{GrayImage, ImageBuffer, RgbaImage};
use libc::{c_uchar, c_uint, c_void, free, size_t};
use std::io::{ErrorKind, Read};
use std::fs::File;
use std::path::Path;
use std::ptr::null_mut;
use std::slice::from_raw_parts;

type LodePNGColorType = u32;
const LCT_GREY: LodePNGColorType = 0;
// const LCT_RGB: LodePNGColorType = 2;
// const LCT_PALETTE: LodePNGColorType = 3;
// const LCT_GREY_ALPHA: LodePNGColorType = 4;
// const LCT_RGBA: LodePNGColorType = 6;

extern {
    fn lodepng_decode_memory(
        outbuf: *mut *mut c_uchar,
        width: *mut c_uint,
        height: *mut c_uint,
        inbuf: *const c_uchar,
        insize: size_t,
        colortype: LodePNGColorType,
        bitdepth: c_uint,
    ) -> c_uint;
    fn lodepng_decode32(
        outbuf: *mut *mut c_uchar,
        width: *mut c_uint,
        height: *mut c_uint,
        inbuf: *const c_uchar,
        insize: size_t,
    ) -> c_uint;
}

pub type Error = std::io::Error;

pub fn load_rgba(path: &Path) -> Result<RgbaImage, Error> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    unsafe {
        let mut width = 0;
        let mut height = 0;
        let mut outbuf = null_mut();
        if lodepng_decode32(
            &mut outbuf, &mut width, &mut height,
            data.as_ptr(), data.len(),
        ) != 0 {
            return Err(Error::new(ErrorKind::Other, "Failed to decode png data"))
        }
        let pixels = from_raw_parts(outbuf as *mut u8, (width * height * 4) as usize).to_vec();
        free(outbuf as *mut c_void);
        match ImageBuffer::from_vec(width, height, pixels) {
            Some(img) => Ok(img),
            None => Err(Error::new(ErrorKind::Other, "Failed to create image buffer")),
        }
    }
}
pub fn load_gray(path: &Path) -> Result<GrayImage, Error> {
    let mut file = File::open(path)?;
    let mut data = Vec::new();
    file.read_to_end(&mut data)?;
    unsafe {
        let mut width = 0;
        let mut height = 0;
        let mut outbuf = null_mut();
        if lodepng_decode_memory(
            &mut outbuf, &mut width, &mut height,
            data.as_ptr(), data.len(),
            LCT_GREY, 8,
        ) != 0 {
            return Err(Error::new(ErrorKind::Other, "Failed to decode png data"))
        }
        let pixels = from_raw_parts(outbuf as *mut u8, (width * height) as usize).to_vec();
        free(outbuf as *mut c_void);
        match ImageBuffer::from_vec(width, height, pixels) {
            Some(img) => Ok(img),
            None => Err(Error::new(ErrorKind::Other, "Failed to create image buffer")),
        }
    }
}

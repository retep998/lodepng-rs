// Copyright © 2015, Peter Atashian

#![feature(collections)]

extern crate image;
extern crate libc;

use image::{ImageBuffer, Rgba};
use libc::{c_uchar, c_uint, c_void, free, size_t};
use std::io::Error as IoError;
use std::io::prelude::*;
use std::fs::File;
use std::mem::zeroed;
use std::path::Path;

extern {
    fn lodepng_decode32(
        outbuf: *mut *mut c_uchar,
        width: *mut c_uint,
        height: *mut c_uint,
        inbuf: *const c_uchar,
        insize: size_t,
    ) -> c_uint;
}

#[derive(Debug)]
pub enum Error {
    Io(IoError),
    Png(&'static str),
}
impl From<IoError> for Error {
    fn from(err: IoError) -> Error {
        Error::Io(err)
    }
}

pub fn load(path: &Path) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, Error> {
    let mut file = try!(File::open(path));
    let mut data = Vec::new();
    try!(file.read_to_end(&mut data));
    unsafe {
        let mut width = zeroed();
        let mut height = zeroed();
        let mut outbuf = zeroed();
        if lodepng_decode32(
            &mut outbuf, &mut width, &mut height,
            data.as_ptr(), data.len() as size_t,
        ) != 0 {
            return Err(Error::Png("Failed to decode png data"))
        }
        let pixels = Vec::from_raw_buf(outbuf as *mut u8, (width * height * 4) as usize);
        free(outbuf as *mut c_void);
        match ImageBuffer::from_vec(width, height, pixels) {
            Some(img) => Ok(img),
            None => Err(Error::Png("Failed to create image buffer")),
        }
    }
}

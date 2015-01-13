// Copyright © 2015, Peter Atashian

#![allow(unstable)]

extern crate image;
extern crate libc;

use image::{ImageBuffer, Rgba};
use libc::{c_uchar, c_uint, c_void, free, size_t};
use std::error::FromError;
use std::io::IoError;
use std::io::fs::File;
use std::mem::zeroed;
use std::result::Result as StdResult;

extern {
    fn lodepng_decode32(
        outbuf: *mut *mut c_uchar,
        width: *mut c_uint,
        height: *mut c_uint,
        inbuf: *const c_uchar,
        insize: size_t,
    ) -> c_uint;
}

#[derive(Show)]
pub enum Error {
    Io(IoError),
    Png(&'static str),
}
impl FromError<IoError> for Error {
    fn from_error(err: IoError) -> Error {
        Error::Io(err)
    }
}
pub type Result<T> = StdResult<T, Error>;

pub fn load(path: &Path) -> Result<ImageBuffer<Vec<u8>, u8, Rgba<u8>>> {
    let mut file = File::open(path);
    let data = try!(file.read_to_end());
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

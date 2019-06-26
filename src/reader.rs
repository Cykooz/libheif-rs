use std::io::{Read, Result as IoResult, Seek, SeekFrom};
use std::os::raw::{c_int, c_void};
use std::slice;

use libheif_sys::*;

use crate::enums::ReaderGrowStatus;

pub trait Reader {
    fn get_position(&mut self) -> IoResult<u64>;

    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize>;

    fn seek(&mut self, position: u64) -> IoResult<u64>;

    fn wait_for_file_size(&mut self, target_size: u64) -> ReaderGrowStatus;
}

#[derive(Debug)]
pub struct StreamReader<T>
where
    T: Read + Seek,
{
    stream: T,
    total_size: u64,
}

impl<T> StreamReader<T>
where
    T: Read + Seek,
{
    pub fn new(stream: T, total_size: u64) -> StreamReader<T> {
        StreamReader { stream, total_size }
    }
}

impl<T> Reader for StreamReader<T>
where
    T: Read + Seek,
{
    fn get_position(&mut self) -> IoResult<u64> {
        self.stream.seek(SeekFrom::Current(0))
    }

    fn read(&mut self, buf: &mut [u8]) -> IoResult<usize> {
        self.stream.read(buf)
    }

    fn seek(&mut self, position: u64) -> IoResult<u64> {
        self.stream.seek(SeekFrom::Start(position as _))
    }

    fn wait_for_file_size(&mut self, target_size: u64) -> ReaderGrowStatus {
        if target_size > self.total_size {
            ReaderGrowStatus::SizeBeyondEof
        } else {
            ReaderGrowStatus::SizeReached
        }
    }
}

unsafe extern "C" fn get_position(user_data: *mut c_void) -> i64 {
    let reader = &mut *(user_data as *mut Box<dyn Reader>);
    match reader.get_position() {
        Ok(v) => v as _,
        Err(_) => -1,
    }
}

unsafe extern "C" fn read(data: *mut c_void, size: usize, user_data: *mut c_void) -> c_int {
    let reader = &mut *(user_data as *mut Box<dyn Reader>);
    let buf = slice::from_raw_parts_mut(data as *mut u8, size);
    match reader.read(buf) {
        Ok(real_size) if real_size == buf.len() => 0,
        _ => 1,
    }
}

unsafe extern "C" fn seek(position: i64, user_data: *mut c_void) -> c_int {
    let reader = &mut *(user_data as *mut Box<dyn Reader>);
    match reader.seek(position as _) {
        Ok(_) => 0,
        Err(_) => 1,
    }
}

unsafe extern "C" fn wait_for_file_size(
    target_size: i64,
    user_data: *mut c_void,
) -> heif_reader_grow_status {
    let reader = &mut *(user_data as *mut Box<dyn Reader>);
    let target_size = target_size as u64;
    reader.wait_for_file_size(target_size) as _
}

pub(crate) const HEIF_READER: heif_reader = heif_reader {
    reader_api_version: 1,
    get_position: Some(get_position),
    read: Some(read),
    seek: Some(seek),
    wait_for_file_size: Some(wait_for_file_size),
};

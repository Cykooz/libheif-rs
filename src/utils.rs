use std::ffi::CStr;
use std::os::raw::c_char;

use libheif_sys as lh;

use crate::FileTypeResult;

#[inline]
pub(crate) fn cstr_to_str<'a>(c_str: *const c_char) -> Option<&'a str> {
    if c_str.is_null() {
        None
    } else {
        let res = unsafe { CStr::from_ptr(c_str).to_str() };
        match res {
            Ok(s) => Some(s),
            Err(_) => None,
        }
    }
}

/// Check file type by it first bytes.
/// Input data should be at least 12 bytes.
pub fn check_file_type(data: &[u8]) -> FileTypeResult {
    let res = unsafe { lh::heif_check_filetype(data.as_ptr(), data.len() as _) };
    num::FromPrimitive::from_u32(res).unwrap_or(FileTypeResult::No)
}

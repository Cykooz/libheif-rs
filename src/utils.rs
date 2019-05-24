use std::os::raw::c_char;
use std::ffi::CStr;

#[inline]
pub fn cstr_to_str<'a>(c_str: *const c_char) -> Option<&'a str> {
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

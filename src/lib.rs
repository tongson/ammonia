use std::ffi::{CStr, CString};
use std::os::raw::c_char;

extern crate ammonia;

#[no_mangle]
pub extern "C" fn clean(h: *const c_char) -> *const c_char {
    let html = unsafe { CStr::from_ptr(h).to_string_lossy().into_owned() };
    let c_str = CString::new(ammonia::clean(&html)).unwrap();
    let ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    return ptr
}

#[no_mangle]
pub extern "C" fn clean_text(h: *const c_char) -> *const c_char {
    let html = unsafe { CStr::from_ptr(h).to_string_lossy().into_owned() };
    let c_str = CString::new(ammonia::clean_text(&html)).unwrap();
    let ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    return ptr
}

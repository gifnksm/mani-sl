use libc::c_int;
use std::ffi::{CString, CStr};

const LC_ALL: c_int = 0;
const LC_COLLATE: c_int = 1;
const LC_CTYPE: c_int = 2;
const LC_MONETARY: c_int = 3;
const LC_NUMERIC: c_int = 4;
const LC_TIME: c_int = 5;
const LC_MESSAGES: c_int = 6;

#[repr(i32)]
#[allow(dead_code)]
pub enum Category {
    All = LC_ALL,
    Collate = LC_COLLATE,
    CType = LC_CTYPE,
    Monetary = LC_MONETARY,
    Numeric = LC_NUMERIC,
    Time = LC_TIME,
    Messages = LC_MESSAGES,
}

mod native {
    use libc::{c_char, c_int};
    extern "C" {
        pub fn setlocale(category: c_int, locale: *const c_char) -> *const c_char;
    }
}

pub fn setlocale(lc: Category, locale: &str) -> String {
    let locale = CString::new(locale.as_bytes()).unwrap();
    unsafe {
        let ret = native::setlocale(lc as c_int, locale.as_ptr());
        String::from_utf8_lossy(&CStr::from_ptr(ret).to_bytes()).to_string()
    }
}

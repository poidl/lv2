extern crate lv2_raw;
use libc;
use std::ffi::CStr;
use std::str;

pub fn mapfeature(hostfeatures: *const (*const lv2_raw::LV2Feature),
                  requiredfeature: &str)
                  -> Result<*mut libc::c_void, &str> {
    let mut x: isize = 0;
    unsafe {
        loop {
            let fptr: *const lv2_raw::LV2Feature = *hostfeatures.offset(x);
            if fptr.is_null() {
                // host doesn't provide feature
                break;
            }
            let s = cstring((*fptr).uri);
            if s == requiredfeature {
                println!{" -> Obtained uri ptr from host: {}", requiredfeature}
                return Ok((*fptr).data);
            }
            x = x + 1;
        }
    }
    println!("Missing feature: {}", requiredfeature);
    Err("missing feature")
}

pub fn cstring<'a>(ptr: *const libc::c_char) -> &'a str {
    unsafe {
        assert!(!ptr.is_null());
        let buf = CStr::from_ptr(ptr).to_bytes();
        let s: &str = str::from_utf8(buf).unwrap();
        s
    }
}

pub fn print_features(features: *const (*const lv2_raw::LV2Feature)) {
    // Print lv2 host features
    let mut x: isize = 0;
    unsafe {
        loop {

            let fptr: *const lv2_raw::LV2Feature = *features.offset(x);
            if fptr.is_null() {
                println!("End of features");
                break;
            }
            let uriptr = (*fptr).uri;
            let buf = CStr::from_ptr(uriptr).to_bytes();
            let s: &str = str::from_utf8(buf).unwrap();
            println!("uri: {}", s);
            x = x + 1;
        }
    }
}

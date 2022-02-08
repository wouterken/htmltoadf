mod adf_builder;
mod adf_structure;
mod extractor;
mod tests;
mod types;

extern crate wasm_bindgen;

pub use adf_builder::convert_html_str_to_adf_str;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
};

#[macro_use]
extern crate lazy_static;

/**
 * If we are compiling as a C Lib (not a WASM lib)
 * we should make sure the function adheres to the C calling convention and its name is not mangled.
 */
#[cfg(not(target_family = "wasm"))]
#[allow(clippy::not_unsafe_ptr_arg_deref)]
#[no_mangle]
pub extern "C" fn convert(html: *const c_char) -> *mut c_char {
    let c_str = unsafe {
        assert!(!html.is_null());
        CStr::from_ptr(html)
    };
    let html = c_str.to_str().unwrap();
    CString::new(convert_html_str_to_adf_str(html.to_string()))
        .unwrap()
        .into_raw()
}

/**
 * If we are compiling for WASM we do not worry about C FFI compatibility
*/
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
#[cfg(target_family = "wasm")]
pub fn convert(html: String) -> String {
    convert_html_str_to_adf_str(html)
}

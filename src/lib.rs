#![allow(non_upper_case_globals)]

use std::{
    env::VarError,
    ffi::{OsStr, OsString},
    os::windows::ffi::OsStringExt,
};

use js_sys::JsString;
use os_str_bytes::RawOsString;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    type Process;
    static process: Process;

    type Env;

    #[wasm_bindgen(method, getter)]
    fn env(this: &Process) -> Env;

    #[wasm_bindgen::prelude::wasm_bindgen(method, structural, indexing_getter)]
    fn get(this: &Env, field: &str) -> Option<JsString>;
}

/// See `std::env::var`
pub fn var<K: AsRef<str>>(key: K) -> Result<String, VarError> {
    let js_str = process
        .env()
        .get(key.as_ref())
        .ok_or(VarError::NotPresent)?;
    if js_str.is_valid_utf16() {
        Ok(js_str.as_string().unwrap())
    } else {
        let vec = Vec::from_iter(js_str.iter().flat_map(u16::to_ne_bytes));
        Err(VarError::NotUnicode(RawOsString::from_raw_vec(vec).unwrap()))
    }
}

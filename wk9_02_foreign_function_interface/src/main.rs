#![allow(non_camel_case_types, dead_code)]

// core idea: how do I talk to other code?

// existing interfaces
// - spawning a process (and communicate via stdin/stdout)
// - be spawned (and communicate via stdin/stdout)
// - file system
// - raw socket
// - HTTP?

/// [EXAMPLE: using libcurl]
use std::ffi::{c_void, CString};
type CURL = c_void;
#[repr(C)]
enum CURLoption {
    CURLOPT_URL = 10002,
}
#[repr(C)]
enum CURLcode {
    CURLE_OK = 0,
    // ...
}
#[link(name = "curl")]
extern "C" {
    fn curl_easy_init() -> *mut CURL;
    fn curl_easy_setopt(handle: *mut CURL, option: CURLoption, value: *mut CURL) -> CURLcode;
    fn curl_easy_perform(easy_handle: *mut CURL) -> CURLcode;
    fn curl_easy_cleanup(handle: *mut CURL);
}

fn main() {
    println!("Hello, world!");

    unsafe {
        let curl = curl_easy_init();
        let url = CString::new("https://insou.dev").unwrap();
        curl_easy_setopt(curl, CURLoption::CURLOPT_URL, url.as_c_str().as_ptr() as _);
        curl_easy_perform(curl);
        curl_easy_cleanup(curl);
    }
}

// Rust dynamically links to libcurl whoaaaa

// Rust bindgen is a create that generates Rust FFI bindings for C (and some C++) libraries
// C bindgen can create C/C++11 headers for Rust libraries which expose a public C API

// but what if I don't want to use C types everywhere?
// just make an abstractino on top -> a safe abstraction on top of unsafe code

extern crate cef_ffi;

use cef_ffi::*;
use std::ptr::null_mut;

fn main() {
    let args = cef_main_args_t {
        argc: 0,
        argv: null_mut(),
    };
    let app = null_mut();
    let sandbox = null_mut();
    unsafe {
        let res = cef_execute_process(&args, app, sandbox);
        println!("Result: {}", res);
    }
    println!("Hello, world!");
}

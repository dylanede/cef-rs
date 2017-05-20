#![feature(proc_macro)]

extern crate proc_macro;

use proc_macro::TokenStream;
use std::str::FromStr;

#[cfg(target_os = "windows")]
fn get_platform_abi_name() -> &'static str {
    //! TODO: Investigate if rustc interprets "stdcall" as "C" for x64.
    "stdcall"
}

#[cfg(any(target_os = "macos", target_os = "linux"))]
fn get_platform_abi_name() -> &'static str {
    "C"
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
fn get_platform_abi_name() -> &'static str {
    panic!("Platform not supported.");
}

#[proc_macro_attribute]
pub fn extern_auto(_: TokenStream, input: TokenStream) -> TokenStream {
    //! TODO: Study and do proper procedural macro error handling.
    let abi = get_platform_abi_name();
    let s = format!("extern \"{}\" {}", abi, input);
    TokenStream::from_str(&s).unwrap()
}

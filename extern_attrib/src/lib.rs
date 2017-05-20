#![feature(proc_macro)]

extern crate proc_macro;
//extern crate syntax;

use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn extern_auto(a: TokenStream, b: TokenStream) -> TokenStream {
    println!("a: {}\nb: {}", a, b);
    b
}


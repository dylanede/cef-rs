//! TODO: Study and do proper procedural macro error handling.
#![feature(proc_macro)]

extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use std::str::FromStr;
use syn::ItemKind;

#[proc_macro_attribute]
pub fn extern_auto(_: TokenStream, input: TokenStream) -> TokenStream {
    let src = input.to_string();
    let item = syn::parse_item(&src);
    let mut item = match item {
        Ok(i) => i,
        Err(_) => {
            // Ignore syntax errors and let the compiler handle it.
            eprintln!("Parse error in extern_auto.");
            return input
        }
    };
    {
        let abi_opt = match item.node {
            ItemKind::Fn(_, _, _, ref mut opt, _, _) => opt,
            _ => return input
        };
        modify_abi(abi_opt);
    }
    let tokens = quote!(#item).to_string();
    println!("tokens: {}", tokens);
    TokenStream::from_str(&tokens).unwrap() // TODO: Handle error.
}

fn modify_abi(abi: &mut Option<syn::Abi>) {
    let name = get_platform_abi_name();
    *abi = Some(syn::Abi::Named(name.into()));
}
    
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

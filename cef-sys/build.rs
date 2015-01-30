#![feature(os, io, path)]

use std::os;
use std::old_io::fs::PathExtensions;

enum Platform {
    Windows,
    Mac,
    Linux
}

fn get_platform() -> Platform {
    match os::getenv("TARGET").unwrap().split('-').nth(2).unwrap() {
        "win32" | "windows" => Platform::Windows,
        "darwin" => Platform::Mac,
        "linux" => Platform::Linux,
        other => panic!("Sorry, platform \"{}\" is not supported by cef-sys.", other)
    }
}

fn main() {
    let dll_name = match get_platform() {
        Platform::Mac => return, // CEF_PATH is not necessarily needed for Mac
        Platform::Windows => "libcef",
        Platform::Linux => "cef"
    };
    let dll_file_name = os::dll_filename(dll_name);
    let cef_dir = Path::new(os::getenv("CEF_PATH")
        .expect("CEF_PATH needs to point to the directory containing the CEF DLL."));
    let cef_path = cef_dir.join(dll_file_name.clone());
    if !cef_path.exists() {
        panic!("Unable to find {} in {}", dll_file_name, cef_dir.as_str().unwrap());
    }
    println!("cargo:rustc-flags=-l {} -L {}", dll_name, cef_dir.as_str().unwrap());
}

/*
fn main() {
    
    let cef_dir = os::getenv("CEF_PATH")
        .expect("CEF_PATH needs to point to the directory containing the CEF DLL.");

    let cef_dir = Path::new(cef_dir);

    let cef_lib = "Chromium Embedded Framework.framework".to_owned();

    let cef_path = cef_dir.join(cef_lib.clone());
    if !cef_path.exists() {
        panic!("Unable to find {} in {}", cef_lib, cef_dir.as_str().unwrap());
    }
    println!("cargo:rustc-flags= -L {}",
             cef_dir.as_str().unwrap());
}
*/

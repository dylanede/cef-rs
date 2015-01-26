use std::os;
use std::io::fs::PathExtensions;
use std::borrow::ToOwned;

#[cfg(target_os="macos")]
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

#[cfg(not(target_os="macos"))]
fn main() {
    let cef_dir = os::getenv("CEF_PATH")
        .expect("CEF_PATH needs to point to the directory containing the CEF DLL.");

    let cef_dir = Path::new(cef_dir);

    let cef_lib = os::dll_filename("cef");

    let cef_path = cef_dir.join(cef_lib.clone());
    if !cef_path.exists() {
        panic!("Unable to find {} in {}", cef_lib, cef_dir.as_str().unwrap());
    }
    println!("cargo:rustc-flags=-l cef -L {}", cef_dir.as_str().unwrap());

}

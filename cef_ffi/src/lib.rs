#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

/// This seems to be required since the build.rs approach
/// "cargo:rustc-link-lib=framework=Chromium Embedded Framework"
/// does not seem to result in any linking attempt to that library.
/// TODO: Do windows and linux need a #[link... attribute too?
/// TODO: Investigate bindgen parameters, the command line version
///       has something related to framework... and linking.
///       Is it present in the library api as well?
#[cfg(target_os="macos")]
#[link(name = "Chromium Embedded Framework", kind = "framework")]
extern {}

#[cfg(target_os="windows")]
#[link(name = "libcef")]
extern {}

#[cfg(target_os="linux")]
#[link(name = "cef")]
extern {}

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));


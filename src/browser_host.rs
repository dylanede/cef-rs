use ffi;
use libc;
use BrowserClientWrapper;
use Browser;
use BrowserClient;
use WindowInfo;
use CefString;
use CefRc;
use State;
use upcast_ptr;
use std::mem::{transmute, zeroed};
use string;
use cast_to_interface;

use std::ptr::null_mut;

pub struct BrowserHost;
pub enum RequestContext {}

impl BrowserHost {
    pub fn create_browser_sync<T: BrowserClient>(
        window_info: &WindowInfo,
        client: T,
        url: &str,
        settings: &BrowserSettings,
        request_context: Option<RequestContext>) -> CefRc<Browser>
    {
        use std::default::Default;
        let info = window_info.to_cef();
        let url = CefString::from_str(url);
        unsafe {
            assert!(ffi::cef_currently_on(ffi::TID_UI) == 1);
            let ptr = ffi::cef_browser_host_create_browser_sync(
                &info as *const _,
                upcast_ptr(BrowserClientWrapper::new(client)),
                string::cast_to_ptr(&url as *const _),
                settings.settings() as *const _,
                zeroed());
            drop(info);
            drop(url);
            if ptr == null_mut() {
                panic!("Generated browser is null!");
            }
            cast_to_interface(ptr)
        }
    }
    pub fn create_browser<T: BrowserClient + Send>(window_info: &WindowInfo,
                                             client: T,
                                             url: &str,
                                             settings: &BrowserSettings,
                                            request_context: Option<RequestContext>) -> bool {
        let info = window_info.to_cef();
        let url = CefString::from_str(url);
        let result = unsafe {
            ffi::cef_browser_host_create_browser(
                &info as *const _,
                upcast_ptr(BrowserClientWrapper::new(client)),
                string::cast_to_ptr(&url as *const _),
                settings.settings() as *const _,
                zeroed()) != 0
        };
        drop(url);
        drop(info);
        result
    }
}

#[repr(C)]
pub struct BrowserSettings {
    pub size: libc::size_t,
    pub windowless_frame_rate: ::libc::c_int,
    pub standard_font_family: CefString,
    pub fixed_font_family: CefString,
    pub serif_font_family: CefString,
    pub sans_serif_font_family: CefString,
    pub cursive_font_family: CefString,
    pub fantasy_font_family: CefString,
    pub default_font_size: ::libc::c_int,
    pub default_fixed_font_size: ::libc::c_int,
    pub minimum_font_size: ::libc::c_int,
    pub minimum_logical_font_size: ::libc::c_int,
    pub default_encoding: CefString,
    pub remote_fonts: State,
    pub javascript: State,
    pub javascript_open_windows: State,
    pub javascript_close_windows: State,
    pub javascript_access_clipboard: State,
    pub javascript_dom_paste: State,
    pub caret_browsing: State,
    pub java: State,
    pub plugins: State,
    pub universal_access_from_file_urls: State,
    pub file_access_from_file_urls: State,
    pub web_security: State,
    pub image_loading: State,
    pub image_shrink_standalone_to_fit: State,
    pub text_area_resize: State,
    pub tab_to_links: State,
    pub local_storage: State,
    pub databases: State,
    pub application_cache: State,
    pub webgl: State,
    pub background_color: ffi::cef_color_t,
}

impl BrowserSettings {
    pub fn new() -> BrowserSettings {
        use std::mem::{zeroed, size_of};
        let mut x: BrowserSettings = unsafe { zeroed() };
        x.size = size_of::<BrowserSettings>() as libc::size_t;
        x
    }
    fn settings<'a>(&'a self) -> &'a ffi::cef_browser_settings_t {
        unsafe { transmute(self) }
    }
}

#[test]
fn check_browser_settings_size() {
    use std::mem::size_of;
    assert!(size_of::<BrowserSettings>() == size_of::<ffi::cef_browser_settings_t>());
}

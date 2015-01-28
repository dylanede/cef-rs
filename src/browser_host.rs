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

pub struct BrowserHost;
pub enum RequestContext {}

impl BrowserHost {
    pub fn create_browser_sync<T: BrowserClient>(window_info: &WindowInfo,
                                             client: CefRc<BrowserClientWrapper<T>>,
                                             url: &CefString,
                                             settings: &BrowserSettings,
                                             request_context: Option<RequestContext>) -> CefRc<Browser> {
        unsafe {
            cast_to_interface(ffi::cef_browser_host_create_browser_sync(
                window_info.info() as *const _,
                upcast_ptr(client),
                string::cast_to_ptr(url as *const _),
                settings.settings() as *const _,
                zeroed()))
        }
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

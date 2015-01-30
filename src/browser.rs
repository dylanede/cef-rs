use ffi;
use CefRc;
use Interface;
use Is;

pub struct Browser {
    vtable: ffi::cef_browser_t
}
unsafe impl Interface<ffi::cef_browser_t> for Browser {}
unsafe impl Is<ffi::cef_base_t> for Browser {}
unsafe impl Is<ffi::cef_browser_t> for Browser {}

impl Browser {

}

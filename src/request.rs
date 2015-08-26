use ffi;
use CefRc;
use Interface;
use RefCountable;
use cast_to_interface;
use cast_from_interface;

#[allow(missing_copy_implementations)]
pub struct Request {
    vtable: ffi::cef_request_t
}

unsafe impl Interface<ffi::cef_request_t> for Request {}
unsafe impl RefCountable for Request {}

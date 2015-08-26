use ffi;
use CefRc;
use Interface;
use RefCountable;
use cast_to_interface;
use cast_from_interface;

#[allow(missing_copy_implementations)]
pub struct Frame {
    vtable: ffi::cef_frame_t
}

unsafe impl Interface<ffi::cef_frame_t> for Frame {}
unsafe impl RefCountable for Frame {}

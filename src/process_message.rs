use ffi;
use CefRc;
use Interface;
use RefCountable;
use cast_to_interface;
use cast_from_interface;

#[allow(missing_copy_implementations)]
pub struct ProcessMessage {
    vtable: ffi::cef_process_message_t
}

unsafe impl Interface<ffi::cef_process_message_t> for ProcessMessage {}
unsafe impl RefCountable for ProcessMessage {}

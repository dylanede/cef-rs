use ffi;
use CefRc;
use Interface;
use RefCountable;
use cast_to_interface;
use cast_from_interface;

#[allow(missing_copy_implementations)]
pub struct DOMNode {
    vtable: ffi::cef_domnode_t
}

unsafe impl Interface<ffi::cef_domnode_t> for DOMNode {}
unsafe impl RefCountable for DOMNode {}

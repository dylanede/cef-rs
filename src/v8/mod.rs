use ffi;
use CefRc;
use Interface;
use RefCountable;
use cast_to_interface;
use cast_from_interface;

#[allow(missing_copy_implementations)]
pub struct V8Context {
    vtable: ffi::cef_v8context_t
}

unsafe impl Interface<ffi::cef_v8context_t> for V8Context {}
unsafe impl RefCountable for V8Context {}

#[allow(missing_copy_implementations)]
pub struct V8StackTrace {
    vtable: ffi::cef_v8stack_trace_t
}

unsafe impl Interface<ffi::cef_v8stack_trace_t> for V8StackTrace {}
unsafe impl RefCountable for V8StackTrace {}

#[allow(missing_copy_implementations)]
pub struct V8Exception {
    vtable: ffi::cef_v8exception_t
}

unsafe impl Interface<ffi::cef_v8exception_t> for V8Exception {}
unsafe impl RefCountable for V8Exception {}

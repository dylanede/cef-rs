use Void;
use Browser;
use Frame;
use Request;
use ProcessMessage;
use ProcessID;
use V8Context;
use V8Exception;
use V8StackTrace;
use DOMNode;
use ListValue;
use CefRc;
use ffi;
use Is;
use RefCountable;

pub trait LoadHandler {}
impl LoadHandler for Void {}

#[derive(NumFromPrimitive)]
#[repr(u32)]
enum NavigationType {
    LinkClicked = 0,
    FormSubmitted,
    BackOrForward,
    Reload,
    FormResubmitted,
    Other
}

#[allow(unused_variables)]
pub trait RenderProcessHandler {
    type OutLoadHandler: LoadHandler = Void;
    fn get_load_handler(&mut self) -> Option<Self::OutLoadHandler> { None }
    fn on_before_navigation(&mut self, browser: &Browser, frame: &Frame, request: &Request, navigation_type: NavigationType, is_redirect: bool) -> bool { false }
    fn on_browser_created(&mut self, browser: &Browser) {}
    fn on_browser_destroyed(&mut self, browser: &Browser) {}
    fn on_context_created(&mut self, browser: &Browser, frame: &Frame, context: &V8Context) {}
    fn on_context_released(&mut self, browser: &Browser, frame: &Frame, context: &V8Context) {}
    fn on_focused_node_changed(&mut self, browser: &Browser, frame: &Frame, node: &DOMNode) {}
    fn on_process_message_received(&mut self, browser: &Browser, source_process: ProcessID, message: &ProcessMessage) -> bool { false }
    fn on_render_thread_created(&mut self, extra_info: &ListValue) {}
    fn on_uncaught_exception(&mut self, browser: &Browser, frame: &Frame, context: &V8Context, exception: &V8Exception, stack_trace: &V8StackTrace) {}
    fn on_webkit_initialized(&mut self) {}
}
impl RenderProcessHandler for Void {}

#[repr(C)]
pub struct RenderProcessHandlerWrapper<T: RenderProcessHandler> {
    vtable: ffi::cef_render_process_handler_t,
    callback: T
}

unsafe impl<T: RenderProcessHandler> Is<ffi::cef_render_process_handler_t> for RenderProcessHandlerWrapper<T> {}
unsafe impl<T: RenderProcessHandler> RefCountable for RenderProcessHandlerWrapper<T> {}

impl<T: RenderProcessHandler> RenderProcessHandlerWrapper<T> {
    pub unsafe fn new(callback: T) -> CefRc<RenderProcessHandlerWrapper<T>> {
        use std::mem::zeroed;
        use cast_to_interface;
        use ::libc::c_int;
        type CSelf = ffi::cef_render_process_handler_t;
        unsafe fn to_self<T: RenderProcessHandler>(_self: &mut CSelf) -> &mut RenderProcessHandlerWrapper<T> {
            use unsafe_downcast_mut;
            unsafe_downcast_mut(_self)
        }
        #[stdcall_win]
        extern fn _1(_self: *mut CSelf) -> *mut ffi::cef_load_handler_t {
            unsafe{ zeroed() }
        }
        #[stdcall_win]
        extern fn _2<T: RenderProcessHandler>(_self: *mut CSelf,
                                              browser: *mut ffi::cef_browser_t,
                                              frame: *mut ffi::cef_frame_t,
                                              request: *mut ffi::cef_request_t,
                                              navigation_type: ffi::cef_navigation_type_t,
                                              is_redirect: c_int) -> c_int
        {
            use ::num::FromPrimitive;
            unsafe {
                to_self::<T>(&mut *_self).callback.on_before_navigation(
                    &cast_to_interface(browser),
                    &cast_to_interface(frame),
                    &cast_to_interface(request),
                    NavigationType::from_u32(navigation_type).unwrap(),
                    is_redirect != 0) as c_int
            }
        }
        #[stdcall_win]
        extern fn _3<T: RenderProcessHandler>(_self: *mut CSelf,
                                              browser: *mut ffi::cef_browser_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_browser_created(
                    &cast_to_interface(browser))
            }
        }
        #[stdcall_win]
        extern fn _4<T: RenderProcessHandler>(_self: *mut CSelf,
                                              browser: *mut ffi::cef_browser_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_browser_destroyed(
                    &cast_to_interface(browser))
            }
        }
        #[stdcall_win]
        extern fn _5<T: RenderProcessHandler>(_self: *mut CSelf,
                                              browser: *mut ffi::cef_browser_t,
                                              frame: *mut ffi::cef_frame_t,
                                              context: *mut ffi::cef_v8context_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_context_created(
                    &cast_to_interface(browser),
                    &cast_to_interface(frame),
                    &cast_to_interface(context))
            }
        }
        #[stdcall_win]
        extern fn _6<T: RenderProcessHandler>(_self: *mut CSelf,
                                              browser: *mut ffi::cef_browser_t,
                                              frame: *mut ffi::cef_frame_t,
                                              context: *mut ffi::cef_v8context_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_context_released(
                    &cast_to_interface(browser),
                    &cast_to_interface(frame),
                    &cast_to_interface(context))
            }
        }
        #[stdcall_win]
        extern fn _7<T: RenderProcessHandler>(_self: *mut CSelf,
                                              browser: *mut ffi::cef_browser_t,
                                              frame: *mut ffi::cef_frame_t,
                                              node: *mut ffi::cef_domnode_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_focused_node_changed(
                    &cast_to_interface(browser),
                    &cast_to_interface(frame),
                    &cast_to_interface(node))
            }
        }
        #[stdcall_win]
        extern fn _8<T: RenderProcessHandler>(_self: *mut CSelf,
                                              browser: *mut ffi::cef_browser_t,
                                              source_process: ffi::cef_process_id_t,
                                              message: *mut ffi::cef_process_message_t) -> c_int
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_process_message_received(
                    &cast_to_interface(browser),
                    ProcessID::from_c(source_process).unwrap(),
                    &cast_to_interface(message)) as c_int
            }
        }
        #[stdcall_win]
        extern fn _9<T: RenderProcessHandler>(_self: *mut CSelf,
                                              extra_info: *mut ffi::cef_list_value_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_render_thread_created(
                    &cast_to_interface(extra_info))
            }
        }
        #[stdcall_win]
        extern fn _10<T: RenderProcessHandler>(_self: *mut CSelf,
                                               browser: *mut ffi::cef_browser_t,
                                               frame: *mut ffi::cef_frame_t,
                                               context: *mut ffi::cef_v8context_t,
                                               exception: *mut ffi::cef_v8exception_t,
                                               stack_trace: *mut ffi::cef_v8stack_trace_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_uncaught_exception(
                    &cast_to_interface(browser),
                    &cast_to_interface(frame),
                    &cast_to_interface(context),
                    &cast_to_interface(exception),
                    &cast_to_interface(stack_trace))
            }
        }
        #[stdcall_win]
        extern fn _11<T: RenderProcessHandler>(_self: *mut CSelf) {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_webkit_initialized()
            }
        }
        CefRc::make(move |base| {
            RenderProcessHandlerWrapper {
                vtable: ffi::cef_render_process_handler_t {
                    base: base,
                    get_load_handler: Some(_1),
                    on_before_navigation: Some(_2::<T>),
                    on_browser_created: Some(_3::<T>),
                    on_browser_destroyed: Some(_4::<T>),
                    on_context_created: Some(_5::<T>),
                    on_context_released: Some(_6::<T>),
                    on_focused_node_changed: Some(_7::<T>),
                    on_process_message_received: Some(_8::<T>),
                    on_render_thread_created: Some(_9::<T>),
                    on_uncaught_exception: Some(_10::<T>),
                    on_web_kit_initialized: Some(_11::<T>)
                },
                callback: callback
            }
        })
    }
}

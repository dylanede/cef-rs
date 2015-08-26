use Void;
use ListValue;
use CefRc;
use ffi;
use Is;
use RefCountable;
use CommandLine;

trait PrintHandler {}
impl PrintHandler for Void {}

#[allow(unused_variables)]
pub trait BrowserProcessHandler {
    type OutPrintHandler = Void;
    fn get_print_handler(&mut self) -> Option<Self::OutPrintHandler> { None }
    fn on_before_child_process_launch(&mut self, command_line: &CommandLine) {}
    fn on_context_initialized(&mut self) {}
    fn on_render_process_thread_created(&mut self, extra_info: &ListValue) {}
}

impl BrowserProcessHandler for Void {}

#[repr(C)]
pub struct BrowserProcessHandlerWrapper<T: BrowserProcessHandler> {
    vtable: ffi::cef_browser_process_handler_t,
    callback: T
}

unsafe impl<T: BrowserProcessHandler> Is<ffi::cef_browser_process_handler_t> for BrowserProcessHandlerWrapper<T> {}
unsafe impl<T: BrowserProcessHandler> RefCountable for BrowserProcessHandlerWrapper<T> {}

impl<T: BrowserProcessHandler> BrowserProcessHandlerWrapper<T> {
    pub unsafe fn new(callback: T) -> CefRc<BrowserProcessHandlerWrapper<T>> {
        use std::mem::zeroed;
        use cast_to_interface;
        type CSelf = ffi::cef_browser_process_handler_t;
        unsafe fn to_self<T: BrowserProcessHandler>(_self: &mut CSelf) -> &mut BrowserProcessHandlerWrapper<T> {
            use unsafe_downcast_mut;
            unsafe_downcast_mut(_self)
        }
        #[stdcall_win]
        extern fn _1(_self: *mut CSelf) -> *mut ffi::cef_print_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _2<T: BrowserProcessHandler>(_self: *mut CSelf, command_line: *mut ffi::cef_command_line_t) {
            unsafe {
                to_self::<T>(&mut *_self).callback
                    .on_before_child_process_launch(&cast_to_interface(command_line))
            }
        }
        #[stdcall_win]
        extern fn _3<T: BrowserProcessHandler>(_self: *mut CSelf) {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_context_initialized()
            }
        }
        #[stdcall_win]
        extern fn _4<T: BrowserProcessHandler>(_self: *mut CSelf, extra_info: *mut ffi::cef_list_value_t) {
            unsafe {
                to_self::<T>(&mut *_self).callback
                    .on_render_process_thread_created(&cast_to_interface(extra_info))
            }
        }
        CefRc::make(move |base| {
            BrowserProcessHandlerWrapper {
                vtable: ffi::cef_browser_process_handler_t {
                    base: base,
                    get_print_handler: Some(_1),
                    on_before_child_process_launch: Some(_2::<T>),
                    on_context_initialized: Some(_3::<T>),
                    on_render_process_thread_created: Some(_4::<T>)
                },
                callback: callback
            }
        })
    }
}

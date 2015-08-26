use ffi;

use Void;
use Is;
use RefCountable;
use CefRc;
use unsafe_downcast_mut;
use upcast_ptr;
use std::mem::zeroed;
pub mod render_process_handler;
use RenderProcessHandler;
use self::render_process_handler::RenderProcessHandlerWrapper;
pub mod browser_process_handler;
use BrowserProcessHandler;
use self::browser_process_handler::BrowserProcessHandlerWrapper;

pub trait ResourceBundleHandler {}
impl ResourceBundleHandler for Void {}

use Interface;
use cast_to_interface;

#[allow(missing_copy_implementations)]
pub struct CommandLine {
    vtable: ffi::cef_command_line_t
}

unsafe impl Interface<ffi::cef_command_line_t> for CommandLine {}
unsafe impl RefCountable for CommandLine {}

#[allow(missing_copy_implementations)]
pub struct SchemeRegistrar {
    vtable: ffi::cef_scheme_registrar_t
}

unsafe impl Interface<ffi::cef_scheme_registrar_t> for SchemeRegistrar {}
unsafe impl RefCountable for SchemeRegistrar {}

#[allow(unused_variables)]
pub trait App : 'static {
    type OutResourceBundleHandler : ResourceBundleHandler = Void;
    type OutBrowserProcessHandler : BrowserProcessHandler = Void;
    type OutRenderProcessHandler : RenderProcessHandler = Void;

    fn on_before_command_line_processing(&mut self,
                                         process_type: &str,
                                         command_line: &CommandLine) {}
    fn on_register_custom_schemes(&mut self, registrar: &SchemeRegistrar) {}
    fn get_resource_bundle_handler(&mut self) -> Option<Self::OutResourceBundleHandler> { None }
    fn get_browser_process_handler(&mut self) -> Option<Self::OutBrowserProcessHandler> { None }
    fn get_render_process_handler(&mut self) -> Option<Self::OutRenderProcessHandler> { None }
}

impl App for Void {}

#[repr(C)]
pub struct AppWrapper<T : App> {
    vtable: ffi::cef_app_t,
    callback: T
}

unsafe impl<T: App> RefCountable for AppWrapper<T> {}
unsafe impl<T: App> Is<ffi::cef_app_t> for AppWrapper<T> {}

impl<T : App> AppWrapper<T> {
    pub unsafe fn new(wrapped: T) -> CefRc<AppWrapper<T>> {
        type CSelf = ffi::cef_app_t;
        unsafe fn to_self<T: App>(_self: &mut CSelf) -> &mut AppWrapper<T> {
            use unsafe_downcast_mut;
            unsafe_downcast_mut(_self)
        }
        #[stdcall_win]
        extern fn _1<T : App>(_self: *mut ffi::cef_app_t,
                        process_type: *const ffi::cef_string_t,
                        command_line: *mut ffi::cef_command_line_t) {
            use ::string::CefString;
            unsafe {
                to_self::<T>(&mut *_self).callback.on_before_command_line_processing(
                    &CefString::view(&*process_type).to_string(), &cast_to_interface(&mut *command_line));
            }
        }
        #[stdcall_win]
        extern fn _2<T : App>(_self: *mut ffi::cef_app_t,
                                     registrar: *mut ffi::cef_scheme_registrar_t) {
            unsafe {
                let this : &mut AppWrapper<T> = unsafe_downcast_mut(&mut *_self);
                this.callback.on_register_custom_schemes(&cast_to_interface(&mut *registrar));
            }
        }
        #[stdcall_win]
        extern fn _3<T : App>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_resource_bundle_handler_t {
            unsafe {
                zeroed()
                //let this : &mut AppWrapper<T> = unsafe_downcast_mut(&mut *_self);
                //this.callback.get_resource_bundle_handler().map(|x| upcast_ptr(x)).unwrap_or_else(|| zeroed())
            }
        }
        #[stdcall_win]
        extern fn _4<T : App>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_browser_process_handler_t {
            unsafe {
                to_self::<T>(&mut *_self).callback.get_browser_process_handler()
                    .map(|x| upcast_ptr(BrowserProcessHandlerWrapper::new(x)))
                    .unwrap_or_else(|| zeroed())
            }
        }
        #[stdcall_win]
        extern fn _5<T : App>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_render_process_handler_t {
            unsafe {
                to_self::<T>(&mut *_self).callback.get_render_process_handler()
                    .map(|x| upcast_ptr(RenderProcessHandlerWrapper::new(x)))
                    .unwrap_or_else(|| zeroed())
            }
        }
        CefRc::make(move |base| AppWrapper {
            vtable: ffi::cef_app_t {
                base: base,
                on_before_command_line_processing: Some(_1::<T>),
                on_register_custom_schemes: Some(_2::<T>),
                get_resource_bundle_handler: Some(_3::<T>),
                get_browser_process_handler: Some(_4::<T>),
                get_render_process_handler: Some(_5::<T>)
            },
            callback: wrapped
        })
    }
}

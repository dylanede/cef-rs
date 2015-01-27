use ffi;

use Is;
use CefRc;
use unsafe_downcast_mut;
use std::mem::zeroed;

#[repr(C)]
pub struct App<T : Callback> {
    vtable: ffi::cef_app_t,
    callback: T
}

impl<T: Callback> Is<ffi::cef_base_t> for App<T> {}
impl<T: Callback> Is<ffi::cef_app_t> for App<T> {}
//impl Is<ffi::cef_resource_bundle_handler_t> for ffi::cef_resource_bundle_handler_t {}
//impl Is<ffi::cef_browser_process_handler_t> for ffi::cef_browser_process_handler_t {}
//impl Is<ffi::cef_render_process_handler_t> for ffi::cef_render_process_handler_t {}

#[allow(unused_variables)]
pub trait Callback {
    type ResourceBundleHandler : Is<ffi::cef_base_t>;// + Is<ffi::cef_resource_bundle_handler_t>;
    type BrowserProcessHandler : Is<ffi::cef_base_t>;// + Is<ffi::cef_browser_process_handler_t>;
    type RenderProcessHandler : Is<ffi::cef_base_t>;// + Is<ffi::cef_render_process_handler_t>;

    fn on_before_command_line_processing(&mut self,
                                         process_type: &ffi::cef_string_t,
                                         command_line: &mut ffi::cef_command_line_t) {}
    fn on_register_custom_schemes(&mut self, registrar: &mut ffi::cef_scheme_registrar_t) {}
    fn get_resource_bundle_handler(&mut self) -> Option<CefRc<Self::ResourceBundleHandler>> { None }
    fn get_browser_process_handler(&mut self) -> Option<CefRc<Self::BrowserProcessHandler>> { None }
    fn get_render_process_handler(&mut self) -> Option<CefRc<Self::RenderProcessHandler>> { None }
}

#[allow(missing_copy_implementations)]
pub struct DefaultCallback;

impl Callback for DefaultCallback {
    type ResourceBundleHandler = ();
    type BrowserProcessHandler = ();
    type RenderProcessHandler = ();
}

impl<T : Callback> App<T> {
    pub fn new(callback: T) -> CefRc<App<T>> {
        extern fn obclp<T : Callback>(_self: *mut ffi::cef_app_t,
                        process_type: *const ffi::cef_string_t,
                        command_line: *mut ffi::cef_command_line_t) {
            unsafe {
                let this : &mut App<T> = unsafe_downcast_mut(&mut *_self);
                this.callback.on_before_command_line_processing(&*process_type, &mut *command_line);
            }
        }
        extern fn orcs<T : Callback>(_self: *mut ffi::cef_app_t,
                                     registrar: *mut ffi::cef_scheme_registrar_t) {
            unsafe {
                let this : &mut App<T> = unsafe_downcast_mut(&mut *_self);
                this.callback.on_register_custom_schemes(&mut *registrar);
            }
        }

        extern fn grbh<T : Callback>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_resource_bundle_handler_t {
            unsafe {
                zeroed()
                //let this : &mut App<T> = unsafe_downcast_mut(&mut *_self);
                //this.callback.get_resource_bundle_handler().map(|x| upcast_ptr(x)).unwrap_or_else(|| zeroed())
            }
        }
        extern fn gbph<T : Callback>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_browser_process_handler_t {
            unsafe {
                zeroed()
                //let this : &mut App<T> = transmute_mut_ref(&mut *_self);
                //this.callback.get_browser_process_handler().map(|x| transmute(x)).unwrap_or_else(|| zeroed())
            }
        }
        extern fn grph<T : Callback>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_render_process_handler_t {
            unsafe {
                zeroed()
                //let this : &mut App<T> = transmute_mut_ref(&mut *_self);
                //this.callback.get_render_process_handler().map(|x| transmute(x)).unwrap_or_else(|| zeroed())
            }
        }
        CefRc::make(move |base| {
            App {
                vtable: ffi::cef_app_t {
                    base: base,
                    on_before_command_line_processing: Some(obclp::<T>),
                    on_register_custom_schemes: Some(orcs::<T>),
                    get_resource_bundle_handler: Some(grbh::<T>),
                    get_browser_process_handler: Some(gbph::<T>),
                    get_render_process_handler: Some(grph::<T>)
                },
                callback: callback
            }
        })
    }
}

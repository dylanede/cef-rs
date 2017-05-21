use ffi;

//use Void;
use Is;
use CefRc;
use unsafe_downcast_mut;
use std::mem::zeroed;
use std::ops::{Deref, DerefMut};
use extern_attrib::extern_auto;

trait ResourceBundleHandler {}
//impl ResourceBundleHandler for Void {}
trait BrowserProcessHandler {}
//impl BrowserProcessHandler for Void {}
trait RenderProcessHandler {}
//impl RenderProcessHandler for Void {}

/// TODO: Refactor to only expose public types, no ffi types, only high-level standard rust types.
//#[allow(unused_variables)]
pub trait App: 'static {
    //type OutResourceBundleHandler : ResourceBundleHandler;
    //type OutBrowserProcessHandler : BrowserProcessHandler;
    //type OutRenderProcessHandler : RenderProcessHandler;

    /// Placeholder.
    fn on_before_command_line_processing(&mut self) {}
    //fn on_before_command_line_processing(&mut self,
    //process_type: &ffi::cef_string_t,
    //command_line: &mut ffi::cef_command_line_t) {}

    /// Placeholder.
    fn on_register_custom_schemes(&mut self) {}
    //fn on_register_custom_schemes(&mut self, registrar: &mut ffi::cef_scheme_registrar_t) {}
    //fn get_resource_bundle_handler(&mut self) -> Option<Self::OutResourceBundleHandler> { None }
    //fn get_browser_process_handler(&mut self) -> Option<Self::OutBrowserProcessHandler> { None }
    //fn get_render_process_handler(&mut self) -> Option<Self::OutRenderProcessHandler> { None }
}

// TODO: Investigate the purpose. Does this work in Rust 2017?
impl App for () {}

#[repr(C)]
pub struct AppWrapper<T: App> {
    vtable: ffi::cef_app_t,
    callback: T,
}

impl<T: App> Deref for AppWrapper<T> {
    type Target = T;
    fn deref<'a>(&'a self) -> &'a T {
        &self.callback
    }
}

impl<T: App> DerefMut for AppWrapper<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.callback
    }
}

unsafe impl<T: App> Is<ffi::cef_base_t> for AppWrapper<T> {}
unsafe impl<T: App> Is<ffi::cef_app_t> for AppWrapper<T> {}

/// TODO: The purpose of this impl should be to hide all ffi details from
///       the caller code and provide a high-level Rust API.
///       For example all cef strings should probably be converted to
///       normal rust String instances.
impl<T: App> AppWrapper<T> {
    pub fn new(wrapped: T) -> CefRc<AppWrapper<T>> {
        #[allow(unused_variables)]
        #[extern_auto]
        fn obclp<T: App>(_self: *mut ffi::cef_app_t,
                         process_type: *const ffi::cef_string_t,
                         command_line: *mut ffi::cef_command_line_t) {
            unsafe {
                let this: &mut AppWrapper<T> = unsafe_downcast_mut(&mut *_self);
                //this.callback.on_before_command_line_processing(&*process_type, &mut *command_line);
                this.callback.on_before_command_line_processing();
            }
        }
        #[allow(unused_variables)]
        #[extern_auto]
        fn orcs<T: App>(_self: *mut ffi::cef_app_t, registrar: *mut ffi::cef_scheme_registrar_t) {
            unsafe {
                let this: &mut AppWrapper<T> = unsafe_downcast_mut(&mut *_self);
                //this.callback.on_register_custom_schemes(&mut *registrar);
                this.callback.on_register_custom_schemes();
            }
        }
        #[extern_auto]
        fn grbh<T: App>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_resource_bundle_handler_t {
            unsafe {
                zeroed()
                //let this : &mut AppWrapper<T> = unsafe_downcast_mut(&mut *_self);
                //this.callback.get_resource_bundle_handler().map(|x| upcast_ptr(x)).unwrap_or_else(|| zeroed())
            }
        }
        #[extern_auto]
        fn gbph<T: App>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_browser_process_handler_t {
            unsafe {
                zeroed()
                //let this : &mut App<T> = transmute_mut_ref(&mut *_self);
                //this.callback.get_browser_process_handler().map(|x| transmute(x)).unwrap_or_else(|| zeroed())
            }
        }
        #[extern_auto]
        fn grph<T: App>(_self: *mut ffi::cef_app_t) -> *mut ffi::cef_render_process_handler_t {
            unsafe {
                zeroed()
                //let this : &mut App<T> = transmute_mut_ref(&mut *_self);
                //this.callback.get_render_process_handler().map(|x| transmute(x)).unwrap_or_else(|| zeroed())
            }
        }
        CefRc::make(move |base| {
            AppWrapper {
                vtable: ffi::cef_app_t {
                    base: base,
                    on_before_command_line_processing: Some(obclp::<T>),
                    on_register_custom_schemes: Some(orcs::<T>),
                    get_resource_bundle_handler: Some(grbh::<T>),
                    get_browser_process_handler: Some(gbph::<T>),
                    get_render_process_handler: Some(grph::<T>),
                },
                callback: wrapped,
            }
        })
    }
}

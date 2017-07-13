/* 
    Many build errors here and the code below seems to use definitions from
    cef-dist/include/internal, TODO: Investigate later.
*/
use ffi::{_cef_client_t, cef_base_ref_counted_t, cef_context_menu_handler_t, cef_dialog_handler_t,
          cef_display_handler_t, cef_download_handler_t, cef_drag_handler_t, cef_find_handler_t,
          cef_focus_handler_t, cef_geolocation_handler_t, cef_jsdialog_handler_t,
          cef_keyboard_handler_t, cef_life_span_handler_t, cef_load_handler_t,
          cef_request_handler_t, cef_browser_t, cef_process_id_t, cef_process_message_t};
use Is;
use CefRc;
use libc;
/*
//use Interface;
//use Void;
*/
use Browser;
use ProcessID;
use ProcessMessage;

/*
//use upcast_ptr;

use extern_macro;

pub mod render_handler;
//pub use self::render_handler::{RenderHandler, RenderHandlerWrapper};
//use self::render_handler::{RenderHandler, RenderHandlerWrapper};

pub struct ContextMenuHandler {}
//impl ContextMenuHandler for Void {}
pub struct DialogHandler {}
//impl DialogHandler for Void {}
pub struct DisplayHandler {}
//impl DisplayHandler for Void {}
pub struct DownloadHandler {}
//impl DownloadHandler for Void {}
pub struct DragHandler {}
//impl DragHandler for Void {}
pub struct FindHandler {}
//impl FindHandler for Void {}
pub struct FocusHandler {}
//impl FocusHandler for Void {}
pub struct GeolocationHandler {}
//impl GeolocationHandler for Void {}
pub struct JSDialogHandler {}
//impl JSDialogHandler for Void {}
pub struct KeyboardHandler {}
//impl KeyboardHandler for Void {}
pub struct LifeSpanHandler {}
//impl LifeSpanHandler for Void {}
pub struct LoadHandler {}
//impl LoadHandler for Void {}
//pub struct RenderHandler {}
//impl RenderHandler for Void {}
pub struct RequestHandler {}
//impl RequestHandler for Void {}
*/

#[allow(unused_variables)]
pub trait BrowserClient: 'static {
    //type OutContextMenuHandler : ContextMenuHandler;
    //type OutDialogHandler : DialogHandler;
    //type OutDisplayHandler : DisplayHandler;
    //type OutDownloadHandler : DownloadHandler;
    //type OutDragHandler : DragHandler;
    //type OutFindHandler : FindHandler;
    //type OutFocusHandler : FocusHandler;
    //type OutGeolocationHandler : GeolocationHandler;
    //type OutJSDialogHandler : JSDialogHandler;
    //type OutKeyboardHandler : KeyboardHandler;
    //type OutLifeSpanHandler : LifeSpanHandler;
    //type OutLoadHandler : LoadHandler;
    //type OutRenderHandler : RenderHandler;
    //type OutRequestHandler : RequestHandler;
/*
    fn get_context_menu_handler(&mut self) -> Option<ContextMenuHandler> {
        None
    }
    fn get_dialog_handler(&mut self) -> Option<DialogHandler> {
        None
    }
    fn get_display_handler(&mut self) -> Option<DisplayHandler> {
        None
    }
    fn get_download_handler(&mut self) -> Option<DownloadHandler> {
        None
    }
    fn get_drag_handler(&mut self) -> Option<DragHandler> {
        None
    }
    fn get_find_handler(&mut self) -> Option<FindHandler> {
        None
    }
    fn get_focus_handler(&mut self) -> Option<FocusHandler> {
        None
    }
    fn get_geolocation_handler(&mut self) -> Option<GeolocationHandler> {
        None
    }
    fn get_jsdialog_handler(&mut self) -> Option<JSDialogHandler> {
        None
    }
    fn get_keyboard_handler(&mut self) -> Option<KeyboardHandler> {
        None
    }
    fn get_life_span_handler(&mut self) -> Option<LifeSpanHandler> {
        None
    }
    fn get_load_handler(&mut self) -> Option<LoadHandler> {
        None
    }
    //fn get_render_handler(&mut self) -> Option<RenderHandler> { None }
    fn get_request_handler(&mut self) -> Option<RequestHandler> {
        None
    }*/

    fn on_process_message_received(
        &mut self,
        browser: &mut Browser,
        source_process: ProcessID,
        message: &mut ProcessMessage,
    ) -> bool {
        false
    }
}

impl BrowserClient for () {}

#[repr(C)]
pub struct BrowserClientWrapper<T: BrowserClient> {
    vtable: _cef_client_t,
    callback: T,
}

unsafe impl<T: BrowserClient> Is<cef_base_ref_counted_t> for BrowserClientWrapper<T> {}
unsafe impl<T: BrowserClient> Is<_cef_client_t> for BrowserClientWrapper<T> {}

/// The *_ffi functions are required to use different calling convensions
/// than normal rust functions, the calling convension is resolved by a macro.
impl<T: BrowserClient> BrowserClientWrapper<T> {
    pub fn new(wrapped: T) -> CefRc<BrowserClientWrapper<T>> {
        use std::mem::zeroed;
        use unsafe_downcast_mut;
        use cast_mut_ref;
        extern_auto_fn!(
            get_context_menu_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_context_menu_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_dialog_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_dialog_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_display_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_display_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_download_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_download_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_drag_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_drag_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_find_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_find_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_focus_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_focus_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_geolocation_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_geolocation_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_jsdialog_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_jsdialog_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_keyboard_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_keyboard_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_life_span_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_life_span_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            get_load_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_load_handler_t
            {
                unsafe { zeroed() }
            }
        );
        // TODO: Fix build errors by removing generics?
        /*
        #[extern_auto]
        fn _13<T : BrowserClient>(_self: *mut _cef_client_t) -> *mut cef_render_handler_t {
            unsafe {
                let this: &mut BrowserClientWrapper<T> = unsafe_downcast_mut(&mut *_self);
                this.callback.get_render_handler()
                    .map(|x| upcast_ptr(RenderHandlerWrapper::new(x)))
                    .unwrap_or_else(|| zeroed())
            }
        }
        */
        extern_auto_fn!(
            get_request_handler_ffi<T: BrowserClient>(_self: *mut _cef_client_t)
                -> *mut cef_request_handler_t
            {
                unsafe { zeroed() }
            }
        );
        extern_auto_fn!(
            on_process_message_received_ffi<T: BrowserClient>(
                _self:          *mut _cef_client_t,
                browser:        *mut cef_browser_t,
                source_process: cef_process_id_t,
                message:        *mut cef_process_message_t)
                -> libc::c_int
            {
                unsafe {
                    let this: &mut BrowserClientWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    this.callback.on_process_message_received(
                        cast_mut_ref(&mut *browser),
                        match source_process {
                            cef_process_id_t::PID_BROWSER => ProcessID::Browser,
                            cef_process_id_t::PID_RENDERER => ProcessID::Renderer,
                            //_ => panic!("Invalid source process ID passed to on_process_message_received by CEF!")
                        },
                        cast_mut_ref(&mut *message)) as libc::c_int
                }
            }
        );
        CefRc::make(move |base| {
            BrowserClientWrapper {
                vtable: _cef_client_t {
                    base: base,
                    get_context_menu_handler: Some(get_context_menu_handler_ffi),
                    get_dialog_handler: Some(get_dialog_handler_ffi),
                    get_display_handler: Some(get_display_handler_ffi),
                    get_download_handler: Some(get_download_handler_ffi),
                    get_drag_handler: Some(get_drag_handler_ffi),
                    get_find_handler: Some(get_find_handler_ffi),
                    get_focus_handler: Some(get_focus_handler_ffi),
                    get_geolocation_handler: Some(get_geolocation_handler_ffi),
                    get_jsdialog_handler: Some(get_jsdialog_handler_ffi),
                    get_keyboard_handler: Some(get_keyboard_handler_ffi),
                    get_life_span_handler: Some(get_life_span_handler_ffi),
                    get_load_handler: Some(get_load_handler_ffi),
                    get_render_handler: None,
                    //get_render_handler: Some(_13::<T>),
                    get_request_handler: Some(get_request_handler_ffi::<T>),
                    on_process_message_received: Some(on_process_message_received_ffi::<T>),
                },
                callback: wrapped,
            }
        })
    }
}

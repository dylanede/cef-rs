/* 
    Many build errors here and the code below seems to use definitions from
    cef-dist/include/internal, TODO: Investigate later.

use ffi;
use Is;
use CefRc;
use libc;

//use Interface;
//use Void;

//use Browser;
use ProcessID;
use ProcessMessage;

//use upcast_ptr;

use extern_attrib::extern_auto;

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
    }
    fn on_process_message_received(&mut self,
                                   browser: &mut Browser,
                                   source_process: ProcessID,
                                   message: &mut ProcessMessage)
                                   -> bool {
        false
    }
}

impl BrowserClient for () {}

#[repr(C)]
pub struct BrowserClientWrapper<T: BrowserClient> {
    vtable: ffi::_cef_client_t,
    callback: T,
}

unsafe impl<T: BrowserClient> Is<ffi::cef_base_ref_counted_t> for BrowserClientWrapper<T> {}
unsafe impl<T: BrowserClient> Is<ffi::_cef_client_t> for BrowserClientWrapper<T> {}

impl<T: BrowserClient> BrowserClientWrapper<T> {
    pub fn new(wrapped: T) -> CefRc<BrowserClientWrapper<T>> {
        use std::mem::zeroed;
        use unsafe_downcast_mut;
        use cast_mut_ref;
        #[extern_auto]
        fn _1(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_context_menu_handler_t {
            //println!("context menu");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _2(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_dialog_handler_t {
            //println!("dialog");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _3(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_display_handler_t {
            //println!("display");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _4(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_download_handler_t {
            //println!("download");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _5(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_drag_handler_t {
            //println!("drag");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _6(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_find_handler_t {
            //println!("find");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _7(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_focus_handler_t {
            //println!("focus");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _8(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_geolocation_handler_t {
            //println!("geo");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _9(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_jsdialog_handler_t {
            //println!("js");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _10(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_keyboard_handler_t {
            //println!("keyboard");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _11(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_life_span_handler_t {
            //println!("lifespan");
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _12(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_load_handler_t {
            //println!("load");
            unsafe { zeroed() }
        }
        // TODO: Fix build errors by removing generics?
        /*
        #[extern_auto]
        fn _13<T : BrowserClient>(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_render_handler_t {
            //println!("render");
            unsafe {
                let this: &mut BrowserClientWrapper<T> = unsafe_downcast_mut(&mut *_self);
                this.callback.get_render_handler()
                    .map(|x| upcast_ptr(RenderHandlerWrapper::new(x)))
                    .unwrap_or_else(|| zeroed())
            }
        }
        */

        #[extern_auto]
        fn _14<T: BrowserClient>(_self: *mut ffi::_cef_client_t) -> *mut ffi::cef_request_handler_t {
            unsafe { zeroed() }
        }
        #[extern_auto]
        fn _15<T: BrowserClient>(_self: *mut ffi::_cef_client_t,
                                 browser: *mut ffi::cef_browser_t,
                                 source_process: ffi::cef_process_id_t,
                                 message: *mut ffi::cef_process_message_t)
                                 -> libc::c_int {
            //println!("message");
            unsafe {
                let this: &mut BrowserClientWrapper<T> = unsafe_downcast_mut(&mut *_self);
                this.callback.on_process_message_received(
                    cast_mut_ref(&mut *browser),
                    match source_process {
                        ffi::PID_BROWSER => ProcessID::Browser,
                        ffi::PID_RENDERER => ProcessID::Renderer,
                        _ => panic!("Invalid source process ID passed to on_process_message_received by CEF!")
                    },
                    cast_mut_ref(&mut *message)) as libc::c_int
            }
        }
        CefRc::make(move |base| {
            BrowserClientWrapper {
                vtable: ffi::_cef_client_t {
                    base: base,
                    get_context_menu_handler: Some(_1),
                    get_dialog_handler: Some(_2),
                    get_display_handler: Some(_3),
                    get_download_handler: Some(_4),
                    get_drag_handler: Some(_5),
                    get_find_handler: Some(_6),
                    get_focus_handler: Some(_7),
                    get_geolocation_handler: Some(_8),
                    get_jsdialog_handler: Some(_9),
                    get_keyboard_handler: Some(_10),
                    get_life_span_handler: Some(_11),
                    get_load_handler: Some(_12),
                    get_render_handler: None,
                    //get_render_handler: Some(_13::<T>),
                    get_request_handler: Some(_14::<T>),
                    on_process_message_received: Some(_15::<T>),
                },
                callback: wrapped,
            }
        })
    }
}
*/

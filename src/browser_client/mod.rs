use ffi;
use Is;
use CefRc;
use libc;

use Interface;
use Void;

use Browser;

pub mod render_handler;
//pub use self::render_handler::{RenderHandler, RenderHandlerWrapper};
use self::render_handler::RenderHandler;

struct ProcessMessage;
unsafe impl Interface<ffi::cef_process_message_t> for ProcessMessage {}

trait ContextMenuHandler {}
impl ContextMenuHandler for Void {}
trait DialogHandler {}
impl DialogHandler for Void {}
trait DisplayHandler {}
impl DisplayHandler for Void {}
trait DownloadHandler {}
impl DownloadHandler for Void {}
trait DragHandler {}
impl DragHandler for Void {}
trait FindHandler {}
impl FindHandler for Void {}
trait FocusHandler {}
impl FocusHandler for Void {}
trait GeolocationHandler {}
impl GeolocationHandler for Void {}
trait JSDialogHandler {}
impl JSDialogHandler for Void {}
trait KeyboardHandler {}
impl KeyboardHandler for Void {}
trait LifeSpanHandler {}
impl LifeSpanHandler for Void {}
trait LoadHandler {}
impl LoadHandler for Void {}
//trait RenderHandler {}
//impl RenderHandler for Void {}
trait RequestHandler {}
impl RequestHandler for Void {}

#[allow(unused_variables)]
pub trait BrowserClient : 'static {
    type OutContextMenuHandler : ContextMenuHandler = ();
    type OutDialogHandler : DialogHandler = ();
    type OutDisplayHandler : DisplayHandler = ();
    type OutDownloadHandler : DownloadHandler = ();
    type OutDragHandler : DragHandler = ();
    type OutFindHandler : FindHandler = ();
    type OutFocusHandler : FocusHandler = ();
    type OutGeolocationHandler : GeolocationHandler = ();
    type OutJSDialogHandler : JSDialogHandler = ();
    type OutKeyboardHandler : KeyboardHandler = ();
    type OutLifeSpanHandler : LifeSpanHandler = ();
    type OutLoadHandler : LoadHandler = ();
    type OutRenderHandler : RenderHandler = ();
    type OutRequestHandler : RequestHandler = ();

    fn get_context_menu_handler(&mut self) -> Option<Self::OutContextMenuHandler> { None }
    fn get_dialog_handler(&mut self) -> Option<Self::OutDialogHandler> { None }
    fn get_display_handler(&mut self) -> Option<Self::OutDisplayHandler> { None }
    fn get_download_handler(&mut self) -> Option<Self::OutDownloadHandler> { None }
    fn get_drag_handler(&mut self) -> Option<Self::OutDragHandler> { None }
    fn get_find_handler(&mut self) -> Option<Self::OutFindHandler> { None }
    fn get_focus_handler(&mut self) -> Option<Self::OutFocusHandler> { None }
    fn get_geolocation_handler(&mut self) -> Option<Self::OutGeolocationHandler> { None }
    fn get_jsdialog_handler(&mut self) -> Option<Self::OutJSDialogHandler> { None }
    fn get_keyboard_handler(&mut self) -> Option<Self::OutKeyboardHandler> { None }
    fn get_life_span_handler(&mut self) -> Option<Self::OutLifeSpanHandler> { None }
    fn get_load_handler(&mut self) -> Option<Self::OutLoadHandler> { None }
    fn get_render_handler(&mut self) -> Option<Self::OutRenderHandler> { None }
    fn get_request_handler(&mut self) -> Option<Self::OutRequestHandler> { None }
    fn on_process_message_received(
        &mut self,
        browser: &mut Browser,
        source_process: ffi::cef_process_id_t,
        message: &mut ProcessMessage) -> bool { false }
}

impl BrowserClient for Void {
    type OutContextMenuHandler = Void;
    type OutDialogHandler = Void;
    type OutDisplayHandler = Void;
    type OutDownloadHandler = Void;
    type OutDragHandler = Void;
    type OutFindHandler = Void;
    type OutFocusHandler = Void;
    type OutGeolocationHandler = Void;
    type OutJSDialogHandler = Void;
    type OutKeyboardHandler = Void;
    type OutLifeSpanHandler = Void;
    type OutLoadHandler = Void;
    type OutRenderHandler = Void;
    type OutRequestHandler = Void;
}

#[repr(C)]
pub struct BrowserClientWrapper<T : BrowserClient> {
    vtable: ffi::cef_client_t,
    callback: T
}

unsafe impl<T: BrowserClient> Is<ffi::cef_base_t> for BrowserClientWrapper<T> {}
unsafe impl<T: BrowserClient> Is<ffi::cef_client_t> for BrowserClientWrapper<T> {}

impl<T : BrowserClient> BrowserClientWrapper<T> {
    pub fn new(wrapped: T) -> CefRc<BrowserClientWrapper<T>> {
        use std::mem::zeroed;
        use unsafe_downcast_mut;
        use cast_mut_ref;
        #[stdcall_win]
        extern fn _1(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_context_menu_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _2(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_dialog_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _3(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_display_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _4(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_download_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _5(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_drag_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _6(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_find_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _7(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_focus_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _8(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_geolocation_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _9(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_jsdialog_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _10(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_keyboard_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _11(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_life_span_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _12(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_load_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _13(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_render_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _14(_self: *mut ffi::cef_client_t) -> *mut ffi::cef_request_handler_t {
            unsafe { zeroed() }
        }
        #[stdcall_win]
        extern fn _15<T : BrowserClient>(_self: *mut ffi::cef_client_t,
                      browser: *mut ffi::cef_browser_t,
                      source_process: ffi::cef_process_id_t,
                      message: *mut ffi::cef_process_message_t) -> libc::c_int {
            unsafe {
                let this: &mut BrowserClientWrapper<T> = unsafe_downcast_mut(&mut *_self);
                this.callback.on_process_message_received(
                    cast_mut_ref(&mut *browser),
                    source_process,
                    cast_mut_ref(&mut *message)) as libc::c_int
            }
        }
        CefRc::make(move |base| {
            BrowserClientWrapper {
                vtable: ffi::cef_client_t {
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
                    get_render_handler: Some(_13),
                    get_request_handler: Some(_14),
                    on_process_message_received: Some(_15::<T>)
                },
                callback: wrapped
            }
        })
    }
}

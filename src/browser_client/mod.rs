/* 
    Many build errors here and the code below seems to use definitions from
    cef-dist/include/internal, TODO: Investigate later.
*/
use Browser;
use CefRc;
use ffi::{_cef_client_t,
          cef_base_ref_counted_t,
          cef_browser_t,
          cef_context_menu_handler_t,
          cef_dialog_handler_t,
          cef_display_handler_t,
          cef_download_handler_t,
          cef_drag_handler_t,
          cef_find_handler_t,
          cef_focus_handler_t,
          cef_geolocation_handler_t,
          cef_jsdialog_handler_t,
          cef_keyboard_handler_t,
          cef_life_span_handler_t,
          cef_load_handler_t,
          cef_process_id_t,
          cef_process_message_t,
          //cef_render_handler_t,
          cef_request_handler_t};
use Is;
use ProcessID;
use ProcessMessage;
use libc;
//use upcast_ptr;

//pub mod render_handler;
//pub use self::render_handler::{RenderHandler, RenderHandlerWrapper};
//use self::render_handler::{RenderHandler, RenderHandlerWrapper};
/*
//use Interface;
//use Void;
*/

/*

use extern_macro;


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
pub struct RenderHandler {}
impl RenderHandler for Void {}
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
*/
    //fn get_render_handler(&mut self) -> Option<OutRenderHandler> { None }
/*
    fn get_request_handler(&mut self) -> Option<RequestHandler> {
        None
    }
*/

    fn on_process_message_received(
        &mut self,
        browser: &mut Browser,
        source_process: ProcessID,
        message: &mut ProcessMessage,
    ) -> bool {
        false
    }
}

// TODO: Investigate later.
impl BrowserClient for () {}

#[repr(C)]
pub struct BrowserClientWrapper<T: BrowserClient> {
    vtable: _cef_client_t,
    callback: T,
}

unsafe impl<T: BrowserClient> Is<cef_base_ref_counted_t> for BrowserClientWrapper<T> {}
unsafe impl<T: BrowserClient> Is<_cef_client_t> for BrowserClientWrapper<T> {}

/// The *_ffi functions are required to use different calling convensions
/// than normal rust functions, the specific calling convension differs
/// depending on platform and is resolved by the extern_auto_fn macro.
/// TODO: Investigate why "extern fn" is not enough in this case.
impl<T: BrowserClient> BrowserClientWrapper<T> {
    pub fn new(wrapped: T) -> CefRc<BrowserClientWrapper<T>> {
        use unsafe_downcast_mut;
        use cast_mut_ref;
        use std::ptr::null_mut;
        extern_auto_fn!(
            get_context_menu_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_context_menu_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_dialog_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_dialog_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_display_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_display_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_download_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_download_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_drag_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_drag_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_find_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_find_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_focus_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_focus_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_geolocation_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_geolocation_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_jsdialog_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_jsdialog_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_keyboard_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_keyboard_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_life_span_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_life_span_handler_t
            {
                null_mut()
            }
        );
        extern_auto_fn!(
            get_load_handler_ffi(_self: *mut _cef_client_t)
                -> *mut cef_load_handler_t
            {
                null_mut()
            }
        );
        /*
        extern_auto_fn!(
            get_render_handler_ffi<T: BrowserClient>(_self: *mut _cef_client_t)
                -> *mut cef_render_handler_t
            {
                unsafe {
                    let this: &mut BrowserClientWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    this.callback.get_render_handler()
                        .map(|x| upcast_ptr(RenderHandlerWrapper::new(x)))
                        .unwrap_or_else(|| null_mut())
                }
            }
        );
	*/
        extern_auto_fn!(
            get_request_handler_ffi<T: BrowserClient>(_self: *mut _cef_client_t)
                -> *mut cef_request_handler_t
            {
                null_mut()
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
                    //get_render_handler: Some(get_render_handler_ffi::<T>),
                    get_request_handler: Some(get_request_handler_ffi::<T>),
                    on_process_message_received: Some(on_process_message_received_ffi::<T>),
                },
                callback: wrapped,
            }
        })
    }
}


/*
Temporary copy-paste from generated bindings and original CEF documentation
for easy access while implementing the related glue code.

#[repr(C)]
#[derive(Copy)]
pub struct _cef_life_span_handler_t {
    pub base: cef_base_ref_counted_t,
    pub on_before_popup: ::std::option::Option<
	unsafe extern "C" fn(
	    self_: *mut _cef_life_span_handler_t,
	    browser: *mut _cef_browser_t,
	    frame: *mut _cef_frame_t,
	    target_url: *const cef_string_t,
	    target_frame_name: *const cef_string_t,
	    target_disposition: cef_window_open_disposition_t,
	    user_gesture: ::std::os::raw::c_int,
	    popupFeatures: *const _cef_popup_features_t,
	    windowInfo: *mut _cef_window_info_t,
	    client: *mut *mut _cef_client_t,
	    settings: *mut _cef_browser_settings_t,
	    no_javascript_access: *mut ::std::os::raw::c_int,
	) -> ::std::os::raw::c_int,
    >,
    pub on_after_created: ::std::option::Option<
	unsafe extern "C" fn(self_: *mut _cef_life_span_handler_t, browser: *mut _cef_browser_t),
    >,
    pub do_close: ::std::option::Option<
	unsafe extern "C" fn(self_: *mut _cef_life_span_handler_t, browser: *mut _cef_browser_t)
	    -> ::std::os::raw::c_int,
    >,
    pub on_before_close: ::std::option::Option<
	unsafe extern "C" fn(self_: *mut _cef_life_span_handler_t, browser: *mut _cef_browser_t),
    >,
}

///
// Implement this structure to handle events related to browser life span. The
// functions of this structure will be called on the UI thread unless otherwise
// indicated.
///
typedef struct _cef_life_span_handler_t {
  ///
  // Base structure.
  ///
  cef_base_ref_counted_t base;

  ///
  // Called on the IO thread before a new popup browser is created. The
  // |browser| and |frame| values represent the source of the popup request. The
  // |target_url| and |target_frame_name| values indicate where the popup
  // browser should navigate and may be NULL if not specified with the request.
  // The |target_disposition| value indicates where the user intended to open
  // the popup (e.g. current tab, new tab, etc). The |user_gesture| value will
  // be true (1) if the popup was opened via explicit user gesture (e.g.
  // clicking a link) or false (0) if the popup opened automatically (e.g. via
  // the DomContentLoaded event). The |popupFeatures| structure contains
  // additional information about the requested popup window. To allow creation
  // of the popup browser optionally modify |windowInfo|, |client|, |settings|
  // and |no_javascript_access| and return false (0). To cancel creation of the
  // popup browser return true (1). The |client| and |settings| values will
  // default to the source browser's values. If the |no_javascript_access| value
  // is set to false (0) the new browser will not be scriptable and may not be
  // hosted in the same renderer process as the source browser. Any
  // modifications to |windowInfo| will be ignored if the parent browser is
  // wrapped in a cef_browser_view_t. Popup browser creation will be canceled if
  // the parent browser is destroyed before the popup browser creation completes
  // (indicated by a call to OnAfterCreated for the popup browser).
  ///
  int(CEF_CALLBACK* on_before_popup)(
      struct _cef_life_span_handler_t* self,
      struct _cef_browser_t* browser,
      struct _cef_frame_t* frame,
      const cef_string_t* target_url,
      const cef_string_t* target_frame_name,
      cef_window_open_disposition_t target_disposition,
      int user_gesture,
      const struct _cef_popup_features_t* popupFeatures,
      struct _cef_window_info_t* windowInfo,
      struct _cef_client_t** client,
      struct _cef_browser_settings_t* settings,
      int* no_javascript_access);

  ///
  // Called after a new browser is created. This callback will be the first
  // notification that references |browser|.
  ///
  void(CEF_CALLBACK* on_after_created)(struct _cef_life_span_handler_t* self,
                                       struct _cef_browser_t* browser);

  ///
  // Called when a browser has recieved a request to close. This may result
  // directly from a call to cef_browser_host_t::*close_browser() or indirectly
  // if the browser is parented to a top-level window created by CEF and the
  // user attempts to close that window (by clicking the 'X', for example). The
  // do_close() function will be called after the JavaScript 'onunload' event
  // has been fired.
  //
  // An application should handle top-level owner window close notifications by
  // calling cef_browser_host_t::try_close_browser() or
  // cef_browser_host_t::CloseBrowser(false (0)) instead of allowing the window
  // to close immediately (see the examples below). This gives CEF an
  // opportunity to process the 'onbeforeunload' event and optionally cancel the
  // close before do_close() is called.
  //
  // When windowed rendering is enabled CEF will internally create a window or
  // view to host the browser. In that case returning false (0) from do_close()
  // will send the standard close notification to the browser's top-level owner
  // window (e.g. WM_CLOSE on Windows, performClose: on OS X, "delete_event" on
  // Linux or cef_window_delegate_t::can_close() callback from Views). If the
  // browser's host window/view has already been destroyed (via view hierarchy
  // tear-down, for example) then do_close() will not be called for that browser
  // since is no longer possible to cancel the close.
  //
  // When windowed rendering is disabled returning false (0) from do_close()
  // will cause the browser object to be destroyed immediately.
  //
  // If the browser's top-level owner window requires a non-standard close
  // notification then send that notification from do_close() and return true
  // (1).
  //
  // The cef_life_span_handler_t::on_before_close() function will be called
  // after do_close() (if do_close() is called) and immediately before the
  // browser object is destroyed. The application should only exit after
  // on_before_close() has been called for all existing browsers.
  //
  // The below examples describe what should happen during window close when the
  // browser is parented to an application-provided top-level window.
  //
  // Example 1: Using cef_browser_host_t::try_close_browser(). This is
  // recommended for clients using standard close handling and windows created
  // on the browser process UI thread. 1.  User clicks the window close button
  // which sends a close notification to
  //     the application's top-level window.
  // 2.  Application's top-level window receives the close notification and
  //     calls TryCloseBrowser() (which internally calls CloseBrowser(false)).
  //     TryCloseBrowser() returns false so the client cancels the window close.
  // 3.  JavaScript 'onbeforeunload' handler executes and shows the close
  //     confirmation dialog (which can be overridden via
  //     CefJSDialogHandler::OnBeforeUnloadDialog()).
  // 4.  User approves the close. 5.  JavaScript 'onunload' handler executes. 6.
  // CEF sends a close notification to the application's top-level window
  //     (because DoClose() returned false by default).
  // 7.  Application's top-level window receives the close notification and
  //     calls TryCloseBrowser(). TryCloseBrowser() returns true so the client
  //     allows the window close.
  // 8.  Application's top-level window is destroyed. 9.  Application's
  // on_before_close() handler is called and the browser object
  //     is destroyed.
  // 10. Application exits by calling cef_quit_message_loop() if no other
  // browsers
  //     exist.
  //
  // Example 2: Using cef_browser_host_t::CloseBrowser(false (0)) and
  // implementing the do_close() callback. This is recommended for clients using
  // non-standard close handling or windows that were not created on the browser
  // process UI thread. 1.  User clicks the window close button which sends a
  // close notification to
  //     the application's top-level window.
  // 2.  Application's top-level window receives the close notification and:
  //     A. Calls CefBrowserHost::CloseBrowser(false).
  //     B. Cancels the window close.
  // 3.  JavaScript 'onbeforeunload' handler executes and shows the close
  //     confirmation dialog (which can be overridden via
  //     CefJSDialogHandler::OnBeforeUnloadDialog()).
  // 4.  User approves the close. 5.  JavaScript 'onunload' handler executes. 6.
  // Application's do_close() handler is called. Application will:
  //     A. Set a flag to indicate that the next close attempt will be allowed.
  //     B. Return false.
  // 7.  CEF sends an close notification to the application's top-level window.
  // 8.  Application's top-level window receives the close notification and
  //     allows the window to close based on the flag from #6B.
  // 9.  Application's top-level window is destroyed. 10. Application's
  // on_before_close() handler is called and the browser object
  //     is destroyed.
  // 11. Application exits by calling cef_quit_message_loop() if no other
  // browsers
  //     exist.
  ///
  int(CEF_CALLBACK* do_close)(struct _cef_life_span_handler_t* self,
                              struct _cef_browser_t* browser);

  ///
  // Called just before a browser is destroyed. Release all references to the
  // browser object and do not attempt to execute any functions on the browser
  // object after this callback returns. This callback will be the last
  // notification that references |browser|. See do_close() documentation for
  // additional usage information.
  ///
  void(CEF_CALLBACK* on_before_close)(struct _cef_life_span_handler_t* self,
                                      struct _cef_browser_t* browser);
} cef_life_span_handler_t;

*/

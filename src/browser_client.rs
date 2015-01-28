use ffi;
use Is;

struct Browser;
struct ProcessMessage;

trait ContextMenuHandler {}
impl ContextMenuHandler for () {}
trait DialogHandler {}
impl DialogHandler for () {}
trait DisplayHandler {}
impl DisplayHandler for () {}
trait DownloadHandler {}
impl DownloadHandler for () {}
trait DragHandler {}
impl DragHandler for () {}
trait FindHandler {}
impl FindHandler for () {}
trait FocusHandler {}
impl FocusHandler for () {}
trait GeolocationHandler {}
impl GeolocationHandler for () {}
trait JSDialogHandler {}
impl JSDialogHandler for () {}
trait KeyboardHandler {}
impl KeyboardHandler for () {}
trait LifeSpanHandler {}
impl LifeSpanHandler for () {}
trait LoadHandler {}
impl LoadHandler for () {}
trait RenderHandler {}
impl RenderHandler for () {}
trait RequestHandler {}
impl RequestHandler for () {}

#[allow(unused_variables)]
pub trait BrowserClient {
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

impl BrowserClient for () {
    type OutContextMenuHandler = ();
    type OutDialogHandler = ();
    type OutDisplayHandler = ();
    type OutDownloadHandler = ();
    type OutDragHandler = ();
    type OutFindHandler = ();
    type OutFocusHandler = ();
    type OutGeolocationHandler = ();
    type OutJSDialogHandler = ();
    type OutKeyboardHandler = ();
    type OutLifeSpanHandler = ();
    type OutLoadHandler = ();
    type OutRenderHandler = ();
    type OutRequestHandler = ();
}

#[repr(C)]
struct BrowserClientWrapper<T : BrowserClient> {
    vtable: ffi::cef_client_t,
    callback: T
}

impl<T: BrowserClient> Is<ffi::cef_base_t> for BrowserClientWrapper<T> {}
impl<T: BrowserClient> Is<ffi::cef_client_t> for BrowserClientWrapper<T> {}

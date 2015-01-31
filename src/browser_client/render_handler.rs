use ffi;
use CefRc;
use Browser;
use CBool;
use Void;

pub type Rect = ffi::cef_rect_t;
pub type Point = ffi::cef_point_t;
pub type Size = ffi::cef_size_t;
pub type CursorHandle = *const ::libc::c_void;
pub type DragOperationsMask = ffi::cef_drag_operations_mask_t;

#[repr(C)]
pub struct DragData {
    vtable: ffi::cef_drag_data_t
}

#[repr(C)]
pub struct ScreenInfo {
    available_rect: Rect,
    depth: i32,
    depth_per_component: i32,
    device_scale_factor: f32,
    is_monochrome: CBool,
    rect: Rect
}

#[test]
fn check_screen_info_size() {
    use std::mem::size_of;
    assert!(size_of::<ScreenInfo>() == size_of::<ffi::cef_screen_info_t>());
}

#[derive(Copy)]
pub enum PaintElementType {
    View,
    Popup
}

#[derive(Copy)]
pub enum CursorDirection {
    East,
    North,
    NorthEast,
    NorthWest,
    South,
    SouthEast,
    SouthWest,
    West
}

#[derive(Copy)]
pub enum CursorBidirection {
    NorthSouth,
    NorthEastSouthWest,
    NorthWestSouthEast
}

pub struct CustomCursorInfo<'a> {
    pub hotspot: Point,
    pub image_scale_factor: f32,
    pub buffer: &'a mut[u8], // size.width * size.height * 4
    pub size: Size,
}
pub enum Cursor<'a> {
    Pointer,
    Cross,
    Hand,
    IBeam,
    Wait,
    Help,
    ResizeDir(CursorDirection),
    ResizeBidir(CursorBidirection),
    ColumnResize,
    RowResize,
    MiddlePanning,
    Panning(CursorDirection),
    Move,
    VerticalText,
    Cell,
    ContextMenu,
    Alias,
    Progress,
    NoDrop,
    Copy,
    None,
    NotAllowed,
    ZoomIn,
    ZoomOut,
    Grab,
    Grabbing,
    Custom(CustomCursorInfo<'a>)
}
#[allow(unused_variables)]
pub trait RenderHandler {
    fn get_root_screen_rect(&mut self, browser: CefRc<Browser>) -> Option<Rect> { None }
    fn get_view_rect(&mut self, browser: CefRc<Browser>) -> Option<Rect>;
    fn get_screen_point(&mut self, browser: CefRc<Browser>, view: (i32, i32)) -> Option<(i32, i32)> { None }
    fn get_screen_info(&mut self, browser: CefRc<Browser>) -> Option<ScreenInfo> { None }
    fn on_popup_show(&mut self, browser: CefRc<Browser>) {}
    fn on_popup_hide(&mut self, browser: CefRc<Browser>) {}
    fn on_popup_size(&mut self, browser: CefRc<Browser>, rect: &Rect) {}
    fn on_paint(&mut self,
                browser: CefRc<Browser>,
                _type: PaintElementType,
                dirty_rects: &[Rect],
                buffer: &[u8], //width * height * 4
                width: i32,
                height: i32);
    fn on_cursor_change(&mut self,
                        browser: CefRc<Browser>,
                        cursor_handle: CursorHandle,
                        cursor: &Cursor) {}
    fn start_dragging(&mut self,
                      browser: CefRc<Browser>,
                      drag_data: CefRc<DragData>,
                      allowed_ops: DragOperationsMask,
                      pos: (i32, i32)) -> bool { false }
    fn update_drag_cursor(&mut self, browser: CefRc<Browser>, operation: DragOperationsMask) {}
    fn on_scroll_offset_changed(&mut self, browser: CefRc<Browser>) {}
}

impl RenderHandler for Void {
    fn get_view_rect(&mut self, _: CefRc<Browser>) -> Option<Rect> {
        unreachable!()
    }
    fn on_paint(&mut self,
                _: CefRc<Browser>,
                _: PaintElementType,
                _: &[Rect],
                _: &[u8], //width * height * 4
                _: i32,
                _: i32) {
        unreachable!()
    }
    
}

pub struct RenderHandlerWrapper;

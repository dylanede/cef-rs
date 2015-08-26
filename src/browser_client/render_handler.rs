use ffi;
use CefRc;
use Browser;
use CBool;
use Void;
use Is;
use RefCountable;
use Interface;

use std::ops::{Deref, DerefMut};

pub type Rect = ffi::cef_rect_t;
pub type Point = ffi::cef_point_t;
pub type Size = ffi::cef_size_t;
pub type CursorHandle = *const ::libc::c_void;
pub type DragOperationsMask = ffi::cef_drag_operations_mask_t;

#[repr(C)]
pub struct DragData {
    vtable: ffi::cef_drag_data_t
}

unsafe impl RefCountable for DragData {}
unsafe impl Interface<ffi::cef_drag_data_t> for DragData {}

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

#[derive(Copy, Clone)]
pub enum PaintElementType {
    View,
    Popup
}

#[derive(Copy, Clone)]
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

#[derive(Copy, Clone)]
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
pub trait RenderHandler : 'static {
    fn get_root_screen_rect(&mut self, browser: &Browser) -> Option<Rect> { None }
    fn get_view_rect(&mut self, browser: &Browser) -> Option<Rect>;
    fn get_screen_point(&mut self, browser: &Browser, view_point: (i32, i32)) -> Option<(i32, i32)> { None }
    fn get_screen_info(&mut self, browser: &Browser) -> Option<ScreenInfo> { None }
    fn on_popup_show(&mut self, browser: &Browser) {}
    fn on_popup_hide(&mut self, browser: &Browser) {}
    fn on_popup_size(&mut self, browser: &Browser, rect: &Rect) {}
    fn on_paint(&mut self,
                browser:&Browser,
                _type: PaintElementType,
                dirty_rects: &[Rect],
                buffer: &[u8], //width * height * 4
                width: i32,
                height: i32);
    fn on_cursor_change(&mut self,
                        browser: &Browser,
                        cursor_handle: CursorHandle,
                        cursor: &Cursor) {}
    fn start_dragging(&mut self,
                      browser: &Browser,
                      drag_data: &DragData,
                      allowed_ops: DragOperationsMask,
                      pos: (i32, i32)) -> bool { false }
    fn update_drag_cursor(&mut self, browser: &Browser, operation: DragOperationsMask) {}
    fn on_scroll_offset_changed(&mut self, browser: &Browser) {}
}

impl RenderHandler for Void {
    fn get_view_rect(&mut self, _: &Browser) -> Option<Rect> {
        unreachable!()
    }
    fn on_paint(&mut self,
                _: &Browser,
                _: PaintElementType,
                _: &[Rect],
                _: &[u8], //width * height * 4
                _: i32,
                _: i32) {
        unreachable!()
    }

}

#[repr(C)]
pub struct RenderHandlerWrapper<T : RenderHandler> {
    vtable: ffi::cef_render_handler_t,
    callback: T
}

impl<T : RenderHandler> Deref for RenderHandlerWrapper<T> {
    type Target = T;
    fn deref<'a>(&'a self) -> &'a T {
        &self.callback
    }
}

impl<T : RenderHandler> DerefMut for RenderHandlerWrapper<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.callback
    }
}

unsafe impl<T: RenderHandler> RefCountable for RenderHandlerWrapper<T> {}
unsafe impl<T: RenderHandler> Is<ffi::cef_render_handler_t> for RenderHandlerWrapper<T> {}

impl<T : RenderHandler> RenderHandlerWrapper<T> {
    pub unsafe fn new(wrapped: T) -> CefRc<RenderHandlerWrapper<T>> {
        use cast_to_interface;
        use std::mem::transmute;
        type CSelf = ffi::cef_render_handler_t;
        unsafe fn to_self<T: RenderHandler>(_self: &mut CSelf) -> &mut RenderHandlerWrapper<T> {
            use unsafe_downcast_mut;
            unsafe_downcast_mut(_self)
        }
        #[stdcall_win]
        extern fn get_root_screen_rect<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            rect: *mut ffi::cef_rect_t) -> ::libc::c_int
        {
            unsafe {
                let rect: &mut ffi::cef_rect_t = transmute(rect);
                match to_self::<T>(&mut *_self).callback.get_root_screen_rect(&cast_to_interface(browser)) {
                    Some(new_rect) => {
                        *rect = new_rect;
                        1
                    },
                    None => 0
                }
            }
        }
        #[stdcall_win]
        extern fn get_view_rect<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            rect: *mut ffi::cef_rect_t) -> ::libc::c_int
        {
            unsafe {
                match to_self::<T>(&mut *_self).callback.get_view_rect(&cast_to_interface(browser)) {
                    Some(new_rect) => {
                        *rect = new_rect;
                        1
                    },
                    None => 0
                }
            }
        }
       #[stdcall_win]
        extern fn get_screen_point<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            view_x: ::libc::c_int,
            view_y: ::libc::c_int,
            screen_x: *mut ::libc::c_int,
            screen_y: *mut ::libc::c_int) -> ::libc::c_int
        {
            unsafe {
                match to_self::<T>(&mut *_self).callback.get_screen_point(&cast_to_interface(browser), (view_x, view_y)) {
                    Some((x, y)) => {
                        *screen_x = x;
                        *screen_y = y;
                        1
                    },
                    None => 0
                }
            }
        }
        #[stdcall_win]
        extern fn get_screen_info<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            screen_info: *mut ffi::cef_screen_info_t) -> ::libc::c_int
        {
            unsafe {
                match to_self::<T>(&mut *_self).callback.get_screen_info(&cast_to_interface(browser)) {
                    Some(info) => {
                        *screen_info = transmute(info);
                        1
                    },
                    None => 0
                }
            }
        }
        #[stdcall_win]
        extern fn on_popup_show<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            show: ::libc::c_int)
        {
            unsafe {
                match show {
                    0 => to_self::<T>(&mut *_self).callback.on_popup_hide(&cast_to_interface(browser)),
                    _ => to_self::<T>(&mut *_self).callback.on_popup_show(&cast_to_interface(browser))
                }
            }
        }
        #[stdcall_win]
        extern fn on_popup_size<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::Struct__cef_browser_t,
            rect: *const ffi::cef_rect_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_popup_size(&cast_to_interface(browser), &*rect)
            }
        }
        #[stdcall_win]
        extern fn on_paint<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            _type: ffi::cef_paint_element_type_t,
            dirty_rects_count: ::libc::size_t,
            dirty_rects: *const ffi::cef_rect_t,
            buffer: *const ::libc::c_void,
            width: ::libc::c_int,
            height: ::libc::c_int)
        {
            use std::slice::from_raw_parts;
            unsafe {
                let _type = match _type {
                    ffi::PET_VIEW => PaintElementType::View,
                    ffi::PET_POPUP => PaintElementType::Popup,
                    _ => unreachable!()
                };
                let dirty_rects = from_raw_parts(dirty_rects, dirty_rects_count as usize);
                let buffer = buffer as *const u8;
                let buffer = from_raw_parts(buffer, (width * height * 4) as usize);
                to_self::<T>(&mut *_self).callback.on_paint(
                    &cast_to_interface(browser),
                    _type, dirty_rects, buffer, width, height);
            }
        }
        #[cfg(not(target_os="linux"))]
        #[stdcall_win]
        extern fn on_cursor_change<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            cursor: *mut ::libc::c_void,
            _type: ffi::cef_cursor_type_t,
            custom_cursor_info: *const ffi::cef_cursor_info_t)
        {

        }
        #[cfg(target_os="linux")]
        extern "C" fn on_cursor_change<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            cursor: ::libc::c_ulong,
            _type: ffi::cef_cursor_type_t,
            custom_cursor_info: *const ffi::cef_cursor_info_t)
        {

        }
        #[stdcall_win]
        extern fn start_dragging<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            drag_data: *mut ffi::cef_drag_data_t,
            allowed_ops: ffi::cef_drag_operations_mask_t,
            x: ::libc::c_int,
            y: ::libc::c_int) -> ::libc::c_int
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.start_dragging(
                    &cast_to_interface(browser),
                    &cast_to_interface(drag_data),
                    allowed_ops,
                    (x, y)) as ::libc::c_int
            }
        }
        #[stdcall_win]
        extern fn update_drag_cursor<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t,
            operation: ffi::cef_drag_operations_mask_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.update_drag_cursor(&cast_to_interface(browser), operation);
            }
        }
        #[stdcall_win]
        extern fn on_scroll_offset_changed<T : RenderHandler>(
            _self: *mut CSelf,
            browser: *mut ffi::cef_browser_t)
        {
            unsafe {
                to_self::<T>(&mut *_self).callback.on_scroll_offset_changed(&cast_to_interface(browser));
            }
        }
        CefRc::make(move |base| RenderHandlerWrapper {
            vtable: ffi::cef_render_handler_t {
                base: base,
                get_root_screen_rect: Some(get_root_screen_rect::<T>),
                get_view_rect: Some(get_view_rect::<T>),
                get_screen_point: Some(get_screen_point::<T>),
                get_screen_info: Some(get_screen_info::<T>),
                on_popup_show: Some(on_popup_show::<T>),
                on_popup_size: Some(on_popup_size::<T>),
                on_paint: Some(on_paint::<T>),
                on_cursor_change: Some(on_cursor_change::<T>),
                start_dragging: Some(start_dragging::<T>),
                update_drag_cursor: Some(update_drag_cursor::<T>),
                on_scroll_offset_changed: Some(on_scroll_offset_changed::<T>)
            },
            callback: wrapped
        })
    }
}

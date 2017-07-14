use ffi;
use CefRc;
use Browser;
use CBool;
//use Void;
use Is;
use Interface;

use unsafe_downcast_mut;
use cast_to_interface;
use std::mem::transmute;

use std::ops::{Deref, DerefMut};

use extern_macro;
use std::os::raw;

pub type Rect = ffi::cef_rect_t;
pub type Point = ffi::cef_point_t;
pub type Size = ffi::cef_size_t;
//pub type CursorHandle = *const ::libc::c_void;
pub type DragOperationsMask = ffi::cef_drag_operations_mask_t;

#[repr(C)]
pub struct DragData {
    vtable: ffi::cef_drag_data_t,
}

unsafe impl Is<ffi::cef_base_ref_counted_t> for DragData {}
unsafe impl Interface<ffi::cef_drag_data_t> for DragData {}

#[repr(C)]
pub struct ScreenInfo {
    available_rect: Rect,
    depth: i32,
    depth_per_component: i32,
    device_scale_factor: f32,
    is_monochrome: CBool,
    rect: Rect,
}

#[test]
fn check_screen_info_size() {
    use std::mem::size_of;
    assert!(size_of::<ScreenInfo>() == size_of::<ffi::cef_screen_info_t>());
}

#[derive(Copy, Clone)]
pub enum PaintElementType {
    View,
    Popup,
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
    West,
}

#[derive(Copy, Clone)]
pub enum CursorBidirection {
    NorthSouth,
    NorthEastSouthWest,
    NorthWestSouthEast,
}

pub struct CustomCursorInfo<'a> {
    pub hotspot: Point,
    pub image_scale_factor: f32,
    pub buffer: &'a mut [u8], // size.width * size.height * 4
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
    Custom(CustomCursorInfo<'a>),
}

#[allow(unused_variables)]
pub trait RenderHandler: 'static {
    fn get_root_screen_rect(&mut self, browser: CefRc<Browser>) -> Option<Rect> {
        None
    }
    fn get_view_rect(&mut self, browser: CefRc<Browser>) -> Option<Rect>;
    fn get_screen_point(&mut self,
                        browser: CefRc<Browser>,
                        view_point: (i32, i32))
                        -> Option<(i32, i32)> {
        None
    }
    fn get_screen_info(&mut self, browser: CefRc<Browser>) -> Option<ScreenInfo> {
        None
    }
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
                        //cursor_handle: CursorHandle,
                        cursor_handle: *mut ::libc::c_void,
                        cursor: &Cursor) {
    }
    fn start_dragging(&mut self,
                      browser: CefRc<Browser>,
                      drag_data: CefRc<DragData>,
                      allowed_ops: DragOperationsMask,
                      pos: (i32, i32))
                      -> bool {
        false
    }
    fn update_drag_cursor(&mut self, browser: CefRc<Browser>, operation: DragOperationsMask) {}
    fn on_scroll_offset_changed(&mut self, browser: CefRc<Browser>) {}
}

/*
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
*/

#[repr(C)]
pub struct RenderHandlerWrapper<T: RenderHandler> {
    vtable: ffi::cef_render_handler_t,
    callback: T,
}

impl<T: RenderHandler> Deref for RenderHandlerWrapper<T> {
    type Target = T;
    fn deref<'a>(&'a self) -> &'a T {
        &self.callback
    }
}

impl<T: RenderHandler> DerefMut for RenderHandlerWrapper<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        &mut self.callback
    }
}

unsafe impl<T: RenderHandler> Is<ffi::cef_base_ref_counted_t> for RenderHandlerWrapper<T> {}
unsafe impl<T: RenderHandler> Is<ffi::cef_render_handler_t> for RenderHandlerWrapper<T> {}

impl<T: RenderHandler> RenderHandlerWrapper<T> {
    pub fn new(wrapped: T) -> CefRc<RenderHandlerWrapper<T>> {
        extern_auto_fn!(
            get_root_screen_rect<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                                      browser: *mut ffi::cef_browser_t,
                                                      rect: *mut ffi::cef_rect_t)
                                                      -> ::libc::c_int {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    let rect: &mut ffi::cef_rect_t = transmute(rect);
                    match this.callback.get_root_screen_rect(browser) {
                        Some(new_rect) => {
                            *rect = new_rect;
                            1
                        }
                        None => 0,
                    }
                }
            }
        );

        extern_auto_fn!(
            get_view_rect<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                               browser: *mut ffi::cef_browser_t,
                                               rect: *mut ffi::cef_rect_t)
                                               -> ::libc::c_int {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    match this.callback.get_view_rect(browser) {
                        Some(new_rect) => {
                            *rect = new_rect;
                            1
                        }
                        None => 0,
                    }
                }
            }
        );

        extern_auto_fn!(
            get_screen_point<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                                  browser: *mut ffi::cef_browser_t,
                                                  view_x: ::libc::c_int,
                                                  view_y: ::libc::c_int,
                                                  screen_x: *mut ::libc::c_int,
                                                  screen_y: *mut ::libc::c_int)
                                                  -> ::libc::c_int {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    match this.callback.get_screen_point(browser, (view_x, view_y)) {
                        Some((x, y)) => {
                            *screen_x = x;
                            *screen_y = y;
                            1
                        }
                        None => 0,
                    }
                }
            }
        );

        extern_auto_fn!(
            get_screen_info<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                                 browser: *mut ffi::cef_browser_t,
                                                 screen_info: *mut ffi::cef_screen_info_t)
                                                 -> ::libc::c_int {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    match this.callback.get_screen_info(browser) {
                        Some(info) => {
                            *screen_info = transmute(info);
                            1
                        }
                        None => 0,
                    }
                }
            }
        );

        extern_auto_fn!(
            on_popup_show<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                               browser: *mut ffi::cef_browser_t,
                                               show: ::libc::c_int) {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    match show {
                        0 => this.callback.on_popup_hide(browser),
                        _ => this.callback.on_popup_show(browser),
                    }
                }
            }
        );

        extern_auto_fn!(
            on_popup_size<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                               browser: *mut ffi::_cef_browser_t,
                                               rect: *const ffi::cef_rect_t) {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    this.callback.on_popup_size(browser, &*rect)
                }
            }
        );

        extern_auto_fn!(
            on_paint<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                          browser: *mut ffi::cef_browser_t,
                                          _type: ffi::cef_paint_element_type_t,
                                          dirty_rects_count: ::libc::size_t,
                                          dirty_rects: *const ffi::cef_rect_t,
                                          buffer: *const raw::c_void,
                                          width: ::libc::c_int,
                                          height: ::libc::c_int) {
                use std::slice::from_raw_parts;
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    let _type = match _type {
                        ffi::cef_paint_element_type_t::PET_VIEW => PaintElementType::View,
                        ffi::cef_paint_element_type_t::PET_POPUP => PaintElementType::Popup,
                        _ => unreachable!(),
                    };
                    let dirty_rects = from_raw_parts(dirty_rects, dirty_rects_count as usize);
                    let buffer = buffer as *const u8;
                    let buffer = from_raw_parts(buffer, (width * height * 4) as usize);
                    this.callback
                        .on_paint(browser, _type, dirty_rects, buffer, width, height);
                }
            }
        );

        /// TODO: Implement me.
        #[allow(unused_variables)]
        #[cfg(not(target_os="linux"))]
        extern_auto_fn!(
            on_cursor_change<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                                  browser: *mut ffi::cef_browser_t,
                                                  cursor: *mut raw::c_void,
                                                  _type: ffi::cef_cursor_type_t,
                                                  custom_cursor_info: *const ffi::cef_cursor_info_t) {
                println!("Unimplemented: on_cursor_change");
            }
        );

        /// TODO: Implement me.
        #[cfg(target_os="linux")]
        extern_auto_fn!(
            on_cursor_change<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                                  browser: *mut ffi::cef_browser_t,
                                                  cursor: ::libc::c_ulong,
                                                  _type: ffi::cef_cursor_type_t,
                                                  custom_cursor_info: *const ffi::cef_cursor_info_t) {
                println!("Unimplemented: on_cursor_change");
            }
        );

        extern_auto_fn!(
            start_dragging<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                                browser: *mut ffi::cef_browser_t,
                                                drag_data: *mut ffi::cef_drag_data_t,
                                                allowed_ops: ffi::cef_drag_operations_mask_t,
                                                x: ::libc::c_int,
                                                y: ::libc::c_int)
                                                -> ::libc::c_int {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    let drag_data: CefRc<DragData> = cast_to_interface(drag_data);
                    this.callback
                        .start_dragging(browser, drag_data, allowed_ops, (x, y)) as
                    ::libc::c_int
                }
            }
        );

        extern_auto_fn!(
            update_drag_cursor<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                                    browser: *mut ffi::cef_browser_t,
                                                    operation: ffi::cef_drag_operations_mask_t) {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    this.callback.update_drag_cursor(browser, operation);
                }
            }
        );

        extern_auto_fn!(
            on_scroll_offset_changed<T: RenderHandler>(_self: *mut ffi::cef_render_handler_t,
                                                          browser: *mut ffi::cef_browser_t,
                                                          x: f64,
                                                          y: f64) {
                unsafe {
                    let this: &mut RenderHandlerWrapper<T> = unsafe_downcast_mut(&mut *_self);
                    let browser: CefRc<Browser> = cast_to_interface(browser);
                    this.callback.on_scroll_offset_changed(browser);
                }
            }
        );

        CefRc::make(move |base| {
            RenderHandlerWrapper {
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
                    on_scroll_offset_changed: Some(on_scroll_offset_changed::<T>),
                    on_ime_composition_range_changed: None,
                },
                callback: wrapped,
            }
        })
    }
}

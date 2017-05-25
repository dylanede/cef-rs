/*
#![feature(box_syntax,
//libc,
           alloc,
           plugin,
           heap_api,
           oom,
           custom_derive,
           associated_type_defaults,
           proc_macro,
)]
*/

//#![plugin(num_macros)]

#![deny(warnings)]

//extern crate cef_sys as ffi;
extern crate cef_ffi as ffi;
//extern crate extern_attrib;
//#[macro_use]
//extern crate bitflags;
//extern crate num; requires nightly
extern crate libc;
//extern crate alloc;
extern crate memalloc;

#[cfg(target_os="windows")]
extern crate kernel32;

/// TODO: Investigate if this was special older versions of Rust.
//pub enum Void {}

use std::mem::{transmute, drop, size_of, zeroed};
use std::ops::{Deref, DerefMut};
use std::default::Default;

//use extern_attrib::extern_auto;
#[macro_use]
mod extern_macro;


use std::ptr::null_mut;

mod app;
pub mod string;
mod browser_client;
mod browser;
mod browser_host;

pub use app::App;
pub use app::AppWrapper;
pub use browser_client::{BrowserClient, BrowserClientWrapper};
/*pub use browser_client::render_handler::{Rect,
                                         Point,
                                         Size,
                                         //CursorHandle,
                                         DragOperationsMask,
                                         RenderHandler,
                                         RenderHandlerWrapper,
                                         PaintElementType,
                                         CursorDirection,
                                         CursorBidirection,
                                         Cursor,
                                         CustomCursorInfo};*/
pub use browser::Browser;
pub use browser_host::BrowserHost;
pub use browser_host::BrowserSettings; /*
pub use browser_host::{event_flags, MouseEvent, MouseButtonType};
pub use browser_host::Modifiers;
pub use browser_host::event_flags::EventFlags;
*/
pub use string::CefString;

pub enum ProcessID {
    Browser,
    Renderer,
}

pub struct ProcessMessage;
unsafe impl Interface<ffi::cef_process_message_t> for ProcessMessage {}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum State {
    Default,
    Enabled,
    Disabled,
}

pub fn shutdown() {
    unsafe { ffi::cef_shutdown() }
}

#[repr(C)]
pub struct CefRc<T: Is<ffi::cef_base_ref_counted_t>> {
    inner: *mut T,
}

pub unsafe trait Is<T> {}
pub unsafe trait Interface<T> {}

unsafe impl Is<ffi::cef_base_ref_counted_t> for ffi::cef_base_ref_counted_t {}
trait CefBase: Is<ffi::cef_base_ref_counted_t> {
    fn add_ref(&mut self);
    fn release(&mut self) -> libc::c_int;
}

impl<T: Is<ffi::cef_base_ref_counted_t>> CefBase for T {
    fn add_ref(&mut self) {
        let base: &mut ffi::cef_base_ref_counted_t = upcast_mut(self);
        unsafe { base.add_ref.unwrap()(base as *mut _) }
    }
    fn release(&mut self) -> libc::c_int {
        let base: &mut ffi::cef_base_ref_counted_t = upcast_mut(self);
        unsafe { base.release.unwrap()(base as *mut _) }
    }
}

impl<T: Is<ffi::cef_base_ref_counted_t>> CefRc<T> {
    fn make<F: FnOnce(ffi::cef_base_ref_counted_t) -> T>(f: F) -> CefRc<T> {
        use std::mem::size_of;
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::Ordering;
        use std::sync::atomic;

        //println!("making {:?}", size_of::<T>());
        #[repr(C)]
        struct RefCounted<T> {
            v: T,
            count: AtomicUsize,
        }
        unsafe impl<T> Is<ffi::cef_base_ref_counted_t> for RefCounted<T> {}

        extern_auto_fn!(add_ref<T>(_self: *mut ffi::cef_base_ref_counted_t) {
            //println!("add {:?}", size_of::<T>());
            let cell: &mut RefCounted<T> = unsafe { unsafe_downcast_mut(&mut *_self) };
            cell.count.fetch_add(1, Ordering::Relaxed);
        });

        extern_auto_fn!(release<T>(_self: *mut ffi::cef_base_ref_counted_t) -> libc::c_int {
            //println!("release {:?}", size_of::<T>());
            unsafe {
                let cell: *mut RefCounted<T> = transmute(_self);
                let old_count = (*cell).count.fetch_sub(1, Ordering::Release);
                if old_count == 1 {
                    //println!("dropping {:?}", size_of::<T>());
                    atomic::fence(Ordering::Acquire);
                    let cell: Box<RefCounted<T>> = transmute(cell);
                    drop(cell);
                }
                if old_count == 1 { 1 } else { 0 }
            }
        });

        extern_auto_fn!(has_one_ref<T>(_self: *mut ffi::cef_base_ref_counted_t) -> libc::c_int {
            let cell: &mut RefCounted<T> = unsafe { unsafe_downcast_mut(&mut *_self) };
            if cell.count.load(Ordering::SeqCst) == 1 {
                1
            } else {
                0
            }
        });

        CefRc {
            inner: unsafe {
                transmute(Box::new(RefCounted {
                                       v: f(ffi::cef_base_ref_counted_t {
                                                size: size_of::<RefCounted<T>>() as libc::size_t,
                                                add_ref: Some(add_ref::<T>),
                                                release: Some(release::<T>),
                                                has_one_ref: Some(has_one_ref::<T>),
                                            }),
                                       count: AtomicUsize::new(1),
                                   }))
            },
        }
    }
    pub fn from_existing(ptr: *mut T) -> CefRc<T> {
        CefRc { inner: ptr }
    }
}

impl<T: Is<ffi::cef_base_ref_counted_t>> Deref for CefRc<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe { &*self.inner }
    }
}

impl<T: Is<ffi::cef_base_ref_counted_t>> DerefMut for CefRc<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe { &mut *self.inner }
    }
}

unsafe fn unsafe_downcast_mut<'a, T1, T2: Is<T1>>(x: &'a mut T1) -> &'a mut T2 {
    transmute(x)
}
fn upcast_mut<'a, T1: Is<T2>, T2>(x: &'a mut T1) -> &'a mut T2 {
    unsafe { transmute(x) }
}

/*
fn upcast<'a, T1: Is<T2>, T2>(x: &'a T1) -> &'a T2 {
    unsafe { transmute(x) }
}
*/

fn upcast_ptr<T1: Is<T2>, T2>(x: CefRc<T1>) -> *mut T2
    where T1: Is<ffi::cef_base_ref_counted_t>
{
    unsafe { transmute(x) }
}

/*
unsafe fn unsafe_downcast_ptr<T1, T2 : Is<T1>>(x: *mut T1) -> CefRc<T2> where T2 : Is<ffi::cef_base_ref_counted_t> {
    transmute(x)
}

fn cast_ref<'a, T1, T2 : Interface<T1>>(x: &'a T1) -> &'a T2 {
    unsafe{ transmute(x) }
}
*/
fn cast_mut_ref<'a, T1, T2: Interface<T1>>(x: &'a mut T1) -> &'a mut T2 {
    unsafe { transmute(x) }
}

fn cast_to_interface<T1, T2: Interface<T1>>(x: *mut T1) -> CefRc<T2>
    where T2: Is<ffi::cef_base_ref_counted_t>
{
    unsafe { transmute(x) }
}
/*
fn cast_from_interface<T1, T2: Interface<T1>>(x: CefRc<T2>) -> *mut T1
    where T2: Is<ffi::cef_base_ref_counted_t>
{
    unsafe { transmute(x) }
}

*/

#[repr(i32)]
#[derive(Copy, Clone)]
pub enum CBool {
    False = 0,
    True = 1,
}
pub use CBool::*;
impl CBool {
    pub fn new(v: bool) -> CBool {
        match v {
            true => True,
            false => False,
        }
    }
    pub fn set(&mut self, v: bool) {
        *self = match v {
            true => True,
            false => False,
        }
    }
    pub fn to_cef(self) -> libc::c_int {
        unsafe { transmute(self) }
    }
}

#[test]
fn check_size_c_bool() {
    use std::mem::size_of;
    assert!(size_of::<CBool>() == size_of::<libc::c_int>());
}

pub struct Settings<'a> {
    pub single_process: bool,
    pub no_sandbox: bool,
    pub browser_subprocess_path: Option<&'a str>,
    pub multi_threaded_message_loop: bool,
    pub windowless_rendering_enabled: bool,
    pub command_line_args_disabled: bool,
    pub cache_path: Option<&'a str>,
    pub persist_session_cookies: bool,
    pub user_agent: Option<&'a str>,
    pub product_version: Option<&'a str>,
    pub locale: Option<&'a str>,
    pub log_file: Option<&'a str>,
    pub log_severity: ffi::cef_log_severity_t,
    pub javascript_flags: Option<&'a str>,
    pub resources_dir_path: Option<&'a str>,
    pub locales_dir_path: Option<&'a str>,
    pub pack_loading_disabled: bool,
    pub remote_debugging_port: Option<i32>,
    pub uncaught_exception_stack_size: Option<i32>,
    pub context_safety_implementation: bool,
    pub ignore_certificate_errors: bool,
    pub background_color: ffi::cef_color_t,
}

impl<'a> Default for Settings<'a> {
    fn default() -> Settings<'a> {
        Settings {
            single_process: false,
            no_sandbox: true,
            browser_subprocess_path: None,
            multi_threaded_message_loop: false,
            windowless_rendering_enabled: false,
            command_line_args_disabled: false,
            cache_path: None,
            persist_session_cookies: false,
            user_agent: None,
            product_version: None,
            locale: None,
            log_file: None,
            log_severity: ffi::cef_log_severity_t::LOGSEVERITY_DEFAULT,
            javascript_flags: None,
            resources_dir_path: None,
            locales_dir_path: None,
            pack_loading_disabled: false,
            remote_debugging_port: None,
            uncaught_exception_stack_size: None,
            context_safety_implementation: false,
            ignore_certificate_errors: false,
            background_color: Default::default(),
        }
    }
}

impl<'a> Settings<'a> {
    fn to_cef(&self) -> ffi::cef_settings_t {
        fn to_cef_str<'a>(s: Option<&'a str>) -> ffi::cef_string_t {
            s.map(|x| CefString::from_str(x).cast())
                .unwrap_or_else(|| unsafe { zeroed() })
        }
        ffi::cef_settings_t {
            size: size_of::<ffi::cef_settings_t>() as libc::size_t,
            single_process: self.single_process as libc::c_int,
            no_sandbox: self.no_sandbox as libc::c_int,
            browser_subprocess_path: to_cef_str(self.browser_subprocess_path),
            multi_threaded_message_loop: self.multi_threaded_message_loop as libc::c_int,
            windowless_rendering_enabled: self.windowless_rendering_enabled as libc::c_int,
            command_line_args_disabled: self.command_line_args_disabled as libc::c_int,
            cache_path: to_cef_str(self.cache_path),
            persist_session_cookies: self.persist_session_cookies as libc::c_int,
            user_agent: to_cef_str(self.user_agent),
            product_version: to_cef_str(self.product_version),
            locale: to_cef_str(self.locale),
            log_file: to_cef_str(self.log_file),
            log_severity: self.log_severity,
            javascript_flags: to_cef_str(self.javascript_flags),
            resources_dir_path: to_cef_str(self.resources_dir_path),
            locales_dir_path: to_cef_str(self.locales_dir_path),
            pack_loading_disabled: self.pack_loading_disabled as libc::c_int,
            remote_debugging_port: self.remote_debugging_port.unwrap_or(0),
            uncaught_exception_stack_size: self.uncaught_exception_stack_size.unwrap_or(0),
            context_safety_implementation: self.context_safety_implementation as libc::c_int,
            ignore_certificate_errors: self.ignore_certificate_errors as libc::c_int,
            background_color: self.background_color,
            accept_language_list: CefString::from_str("").cast(),
            enable_net_security_expiration: 1,
            external_message_pump: 0,
            framework_dir_path: CefString::from_str("").cast(),
            persist_user_preferences: 0,
            user_data_path: CefString::from_str("").cast(),
        }
    }
}

pub struct WindowInfo<'a> {
    pub window_name: Option<&'a str>,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub windowless_rendering_enabled: bool,
    pub transparent_painting_enabled: bool,
}

impl<'a> Default for WindowInfo<'a> {
    fn default() -> WindowInfo<'a> {
        WindowInfo {
            window_name: None,
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            windowless_rendering_enabled: false,
            transparent_painting_enabled: false,
        }
    }
}

impl<'a> WindowInfo<'a> {
    #[cfg(target_os="linux")]
    fn to_cef(&self) -> ffi::cef_window_info_t {
        use std::default::Default;
        let mut info: ffi::cef_window_info_t = Default::default();
        info.x = self.x as u32;
        info.y = self.y as u32;
        info.width = self.width as u32;
        info.height = self.height as u32;
        info.windowless_rendering_enabled = CBool::new(self.windowless_rendering_enabled).to_cef();
        info.transparent_painting_enabled = CBool::new(self.transparent_painting_enabled).to_cef();
        info
    }
    #[cfg(target_os="macos")]
    fn to_cef(&self) -> ffi::cef_window_info_t {
        //use std::default::Default;
        ffi::cef_window_info_t {
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            windowless_rendering_enabled: CBool::new(self.windowless_rendering_enabled).to_cef(),
            transparent_painting_enabled: CBool::new(self.transparent_painting_enabled).to_cef(),
            window_name: match self.window_name {
                Some(name) => CefString::from_str(name).cast(),
                _ => CefString::from_str("").cast(),
            },
            hidden: 0,
            parent_view: null_mut(),
            view: null_mut(),
        }
    }
    #[cfg(target_os="windows")]
    fn to_cef(&self) -> ffi::cef_window_info_t {
        //use std::default::Default;
        use std::ptr::null_mut;
        ffi::cef_window_info_t {
            ex_style: 0x00000000,
            window_name: CefString::from_str("CEF").cast(),
            style: 0x16CF0000,
            x: self.x,
            y: self.y,
            width: self.width,
            height: self.height,
            parent_window: null_mut(),
            menu: null_mut(),
            windowless_rendering_enabled: CBool::new(self.windowless_rendering_enabled).to_cef(),
            transparent_painting_enabled: CBool::new(self.transparent_painting_enabled).to_cef(),
            window: null_mut(),
        }
        /* TODO: cef_window_info_t does not implement Default, is it supposed to?
        let mut info: ffi::cef_window_info_t = Default::default();
        info.x = self.x;
        info.y = self.y;
        info.width = self.width;
        info.height = self.height;
        info.windowless_rendering_enabled = CBool::new(self.windowless_rendering_enabled).to_cef();
        info.transparent_painting_enabled = CBool::new(self.transparent_painting_enabled).to_cef();
        if !self.windowless_rendering_enabled {
            // WS_OVERLAPPEDWINDOW | WS_CLIPCHILDREN | WS_CLIPSIBLINGS | WS_VISIBLE
            info.style = 0x16CF0000;
        }
        if let Some(name) = self.window_name {
            info.window_name = CefString::from_str(name).cast()
        }
        info*/
    }
}

#[cfg(not(target_os="windows"))]
fn with_args<T, F: FnOnce(ffi::cef_main_args_t) -> T>(f: F) -> T {
    use std::ffi::CString;
    let args: Vec<CString> = std::env::args().map(|x| CString::new(x).unwrap()).collect();
    println!("{:?}", args);
    let args: Vec<*mut libc::c_char> = args.iter().map(|x| x.as_ptr() as *mut _).collect();
    let args = &args;
    let args_ = ffi::cef_main_args_t {
        argc: args.len() as libc::c_int,
        argv: args.as_ptr() as *mut _,
    };
    let result = f(args_);
    drop(args);
    result
}

#[cfg(target_os="windows")]
fn with_args<T, F: FnOnce(ffi::cef_main_args_t) -> T>(f: F) -> T {
    use std::ptr::null;
    let args_ = ffi::cef_main_args_t {
        instance: unsafe { kernel32::GetModuleHandleW(null()) } as ffi::HINSTANCE,
    };
    f(args_)
}

/// TODO: Investigate issue #36887 <https://github.com/rust-lang/rust/issues/36887>
pub fn execute_process<T: App>(app: Option<T>) -> isize {
    with_args(move |args| unsafe {
                  ffi::cef_execute_process(&args as *const _,
                                           app.map(|x| upcast_ptr(AppWrapper::new(x)))
                                               .unwrap_or_else(|| zeroed()),
                                           zeroed()) as isize
              })
}

/// TODO: Investigate issue #36887 <https://github.com/rust-lang/rust/issues/36887>
pub fn initialize<T: App>(settings: &Settings, app: Option<T>) -> bool {
    let settings = settings.to_cef();
    let result = with_args(move |args| unsafe {
                               ffi::cef_initialize(&args as *const _,
                                                   &settings as *const _,
                                                   app.map(|x| upcast_ptr(AppWrapper::new(x)))
                                                       .unwrap_or_else(|| zeroed()),
                                                   zeroed())
                           });
    drop(settings);
    match result {
        0 => false,
        _ => true,
    }
}

pub fn run_message_loop() {
    unsafe { ffi::cef_run_message_loop() }
}

pub fn do_message_loop_work() {
    unsafe { ffi::cef_do_message_loop_work() }
}

impl<T: Is<ffi::cef_base_ref_counted_t>> Drop for CefRc<T> {
    fn drop(&mut self) {
        unsafe {
            if self.inner != std::ptr::null_mut() {
                (*self.inner).release();
                self.inner = std::ptr::null_mut();
            }
        };
    }
}

impl<T: Is<ffi::cef_base_ref_counted_t>> Clone for CefRc<T> {
    fn clone(&self) -> CefRc<T> {
        unsafe { (*self.inner).add_ref() };
        CefRc { inner: self.inner }
    }
}

#![feature(unsafe_destructor, box_syntax, libc, alloc, core, collections, os, std_misc, plugin)]

extern crate "cef-sys" as ffi;
#[plugin]
#[no_link]
extern crate callc;
extern crate libc;
extern crate alloc;

use std::mem::{transmute, drop, size_of, zeroed};
use std::ops::{Deref, DerefMut};

mod app;
pub mod string;
mod browser_client;
mod browser;
mod browser_host;

pub use app::App;
pub use app::AppWrapper;
pub use browser_client::BrowserClient;
pub use browser_client::BrowserClientWrapper;
pub use browser::Browser;
pub use browser_host::BrowserHost;
pub use browser_host::BrowserSettings;
pub use string::CefString;

#[repr(C)]
#[derive(Copy)]
pub enum State {
    Default,
    Enabled,
    Disabled
}

pub fn shutdown() {
    unsafe { ffi::cef_shutdown() }
}

unsafe fn unsafe_downcast_mut<'a, T1, T2 : Is<T1>>(x: &'a mut T1) -> &'a mut T2 {
    transmute(x)
}
fn upcast_mut<'a, T1 : Is<T2>, T2>(x: &'a mut T1) -> &'a mut T2 {
    unsafe{ transmute(x) }
}
fn upcast<'a, T1 : Is<T2>, T2>(x: &'a T1) -> &'a T2 {
    unsafe{ transmute(x) }
}

fn upcast_ptr<T1 : Is<T2>, T2>(x: CefRc<T1>) -> *mut T2 where T1 : Is<ffi::cef_base_t> {
    unsafe { transmute(x) }
}

unsafe fn unsafe_downcast_ptr<T1, T2 : Is<T1>>(x: *mut T1) -> CefRc<T2> where T2 : Is<ffi::cef_base_t> {
    transmute(x)
}

fn cast_ref<'a, T1, T2 : Interface<T1>>(x: &'a T1) -> &'a T2 {
    unsafe{ transmute(x) }
}

fn cast_mut_ref<'a, T1, T2 : Interface<T1>>(x: &'a mut T1) -> &'a mut T2 {
    unsafe{ transmute(x) }
}

unsafe fn cast_to_interface<T1, T2 : Interface<T1>>(x: *mut T1) -> CefRc<T2> where T2 : Is<ffi::cef_base_t> {
    transmute(x)
}

#[repr(C)]
#[unsafe_no_drop_flag]
pub struct CefRc<T: Is<ffi::cef_base_t>> {
    inner: *mut T
}

pub unsafe trait Is<T> {}
pub unsafe trait Interface<T> {}

unsafe impl Is<ffi::cef_base_t> for ffi::cef_base_t {}
unsafe impl<T> Is<T> for () {}
trait CefBase : Is<ffi::cef_base_t> {
    fn add_ref(&mut self);
    fn release(&mut self) -> libc::c_int;
}

impl<T: Is<ffi::cef_base_t>> CefBase for T {
    fn add_ref(&mut self) {
        let base: &mut ffi::cef_base_t = upcast_mut(self);
        base.add_ref.unwrap()(base as *mut _)
    }
    fn release(&mut self) -> libc::c_int {
        let base: &mut ffi::cef_base_t = upcast_mut(self);
        base.release.unwrap()(base as *mut _)
    }
}

impl<T: Is<ffi::cef_base_t>> CefRc<T> {
    fn make<F: FnOnce(ffi::cef_base_t) -> T>(f: F) -> CefRc<T> {
        use std::sync::atomic::AtomicUsize;
        use std::sync::atomic::Ordering;
        use std::sync::atomic;
        #[repr(C)]
        struct RefCounted<T> {
            v: T,
            count: AtomicUsize
        }
        unsafe impl<T> Is<ffi::cef_base_t> for RefCounted<T> {}

        #[stdcall_win]
        extern "C" fn add_ref<T>(_self: *mut ffi::cef_base_t) {
            let cell: &mut RefCounted<T> = unsafe{ unsafe_downcast_mut(&mut *_self) };
            cell.count.fetch_add(1, Ordering::Relaxed);
        }
        #[stdcall_win]
        extern "C" fn release<T>(_self: *mut ffi::cef_base_t) -> libc::c_int {
            unsafe {
                let cell: *mut RefCounted<T> = transmute(_self);
                let old_count = (*cell).count.fetch_sub(1, Ordering::Release);
                if old_count == 1 {
                    atomic::fence(Ordering::Acquire);
                    let cell: Box<RefCounted<T>> = transmute(cell);
                    drop(cell);
                }
                if old_count == 1 { 1 } else { 0 }
            }
        }
        #[stdcall_win]
        extern "C" fn has_one_ref<T>(_self: *mut ffi::cef_base_t) -> libc::c_int {
            let cell: &mut RefCounted<T> = unsafe{ unsafe_downcast_mut(&mut *_self) };
            if cell.count.load(Ordering::SeqCst) == 1 { 1 } else { 0 }
        }
        CefRc {
            inner: unsafe { transmute(box RefCounted {
                v: f(ffi::cef_base_t {
                    size: size_of::<RefCounted<T>>() as libc::size_t,
                    add_ref: Some(add_ref::<T>),
                    release: Some(release::<T>),
                    has_one_ref: Some(has_one_ref::<T>)
                }),
                count: AtomicUsize::new(1)
            })}
        }
    }
    pub fn from_existing(ptr: *mut T) -> CefRc<T> {
        CefRc { inner: ptr }
    }
}

impl<T: Is<ffi::cef_base_t>> Deref for CefRc<T> {
    type Target = T;

    fn deref<'a>(&'a self) -> &'a T {
        unsafe{ &*self.inner }
    }
}

impl<T: Is<ffi::cef_base_t>> DerefMut for CefRc<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut T {
        unsafe{ &mut *self.inner }
    }
}

pub fn execute_process<T : App>(app: Option<CefRc<AppWrapper<T>>>) -> libc::c_int {
    use std::ffi::CString;
    let args: Vec<CString> = std::os::args().into_iter().map(|x| CString::from_vec(x.into_bytes())).collect();
    let args: Vec<*mut libc::c_char> = args.iter().map(|x| x.as_slice_with_nul().as_ptr() as *mut _).collect();
    let args = &args[];
    let args = ffi::cef_main_args_t { argc: args.len() as libc::c_int, argv: args[].as_ptr() as *mut _ };
    unsafe{
        ffi::cef_execute_process(
            &args as *const _,
            app.map(|x| upcast_ptr(x)).unwrap_or_else(|| zeroed()),
            zeroed())
    }
}

#[repr(i32)]
#[derive(Copy)]
pub enum CBool {
    False = 0,
    True = 1
}
pub use CBool::*;
impl CBool {
    pub fn new(v: bool) -> CBool {
        match v {
            true => True,
            false => False
        }
    }
    pub fn set(&mut self, v: bool) {
        *self = match v {
            true => True,
            false => False
        }
    }
}

#[test]
fn check_size_c_bool() {
    use std::mem::size_of;
    assert!(size_of::<CBool>() == size_of::<libc::c_int>());
}

#[repr(C)]
pub struct Settings {
    pub size: ::libc::size_t,
    pub single_process: CBool,
    pub no_sandbox: CBool,
    pub browser_subprocess_path: CefString,
    multi_threaded_message_loop: CBool,
    windowless_rendering_enabled: CBool,
    command_line_args_disabled: CBool,
    pub cache_path: CefString,
    persist_session_cookies: CBool,
    pub user_agent: CefString,
    pub product_version: CefString,
    pub locale: CefString,
    pub log_file: CefString,
    pub log_severity: ffi::cef_log_severity_t,
    pub javascript_flags: CefString,
    pub resources_dir_path: CefString,
    pub locales_dir_path: CefString,
    pub pack_loading_disabled: CBool,
    pub remote_debugging_port: ::libc::c_int,
    pub uncaught_exception_stack_size: ::libc::c_int,
    pub context_safety_implementation: CBool,
    pub ignore_certificate_errors: CBool,
    pub background_color: ffi::cef_color_t,
}

unsafe impl Is<ffi::cef_settings_t> for Settings {}

impl Settings {
    pub fn new() -> Settings {
        let mut x: Settings = unsafe { zeroed() };
        x.size = size_of::<ffi::cef_settings_t>() as libc::size_t;
        x.no_sandbox = True;
        //x.command_line_args_disabled = 1;
        x
    }
    fn settings<'a>(&'a self) -> &'a ffi::cef_settings_t {
        upcast(self)
    }

    pub fn set_windowless_rendering(&mut self, enabled: bool) {
        self.windowless_rendering_enabled = CBool::new(enabled);
    }
}

#[test]
fn settings_size_check() {
    use std::mem::size_of;
    assert!(size_of::<Settings>() == size_of::<ffi::cef_settings_t>());
}

#[repr(C)]
pub struct WindowInfo {
    pub window_name: CefString,
    pub x: ::libc::c_int,
    pub y: ::libc::c_int,
    pub width: ::libc::c_int,
    pub height: ::libc::c_int,
    pub hidden: CBool,
    pub parent_view: *mut ::libc::c_void,
    pub windowless_rendering_enabled: CBool,
    pub transparent_painting_enabled: CBool,
    pub view: *mut ::libc::c_void,
}
unsafe impl Is<ffi::cef_window_info_t> for WindowInfo {}

#[test]
fn window_size_check() {
    use std::mem::size_of;
    assert!(size_of::<WindowInfo>() == size_of::<ffi::cef_window_info_t>());
}

impl WindowInfo {
    pub fn new() -> WindowInfo {
        let x: WindowInfo = unsafe { zeroed() };
        x
    }
    fn info<'a>(&'a self) -> &'a ffi::cef_window_info_t {
        upcast(self)
    }
}

pub fn initialize<T : App>(settings: &Settings, app: Option<CefRc<AppWrapper<T>>>) -> bool {
    use std::ffi::CString;
    let args: Vec<CString> = std::os::args().into_iter().map(|x| CString::from_vec(x.into_bytes())).collect();
    let args: Vec<*mut libc::c_char> = args.iter().map(|x| x.as_slice_with_nul().as_ptr() as *mut _).collect();
    let args = &args[];
    let args = ffi::cef_main_args_t { argc: args.len() as libc::c_int, argv: args[].as_ptr() as *mut _ };
    let result = unsafe{
        ffi::cef_initialize(
            &args as *const _,
            settings.settings() as *const _,
            app.map(|x| upcast_ptr(x)).unwrap_or_else(|| zeroed()), zeroed()) };
    match result {
        0 => false,
        _ => true
    }
}

pub fn run_message_loop() {
    unsafe { ffi::cef_run_message_loop() }
}

#[unsafe_destructor]
impl<T: Is<ffi::cef_base_t>> Drop for CefRc<T> {
    fn drop(&mut self) {
        unsafe{
            if self.inner != std::ptr::null_mut() {
                (*self.inner).release();
                self.inner = std::ptr::null_mut();
            }
        };
    }
}

impl<T: Is<ffi::cef_base_t>> Clone for CefRc<T> {
    fn clone(&self) -> CefRc<T> {
        unsafe{ (*self.inner).add_ref() };
        CefRc { inner: self.inner }
    }
}

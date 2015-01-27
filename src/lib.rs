#![feature(unsafe_destructor, box_syntax)]

extern crate "cef-sys" as ffi;
extern crate libc;
extern crate alloc;

use std::mem::{transmute, drop, size_of, forget, zeroed};
use std::ops::{Deref, DerefMut};
use std::borrow::ToOwned;

pub mod app;
pub mod string;

pub use app::App;
pub use string::CefString;

pub enum Void {}

unsafe fn unsafe_downcast_mut<'a, T1, T2 : Is<T1>>(x: &'a mut T1) -> &'a mut T2 {
    transmute(x)
}
fn upcast_mut<'a, T1 : Is<T2>, T2>(x: &'a mut T1) -> &'a mut T2 {
    unsafe{ transmute(x) }
}

fn upcast_ptr<T1 : Is<T2>, T2>(x: CefRc<T1>) -> *mut T2 where T1 : Is<ffi::cef_base_t> {
    unsafe {
        let result = transmute(x.inner);
        forget(x);
        result
    }
}

/*
unsafe fn transmute_mut_ref<'a, T1, T2>(x: &'a mut T1) -> &'a mut T2 {
    transmute(x)
}
unsafe fn transmute_ref<'a, T1, T2>(x: &'a T1) -> &'a T2 {
    transmute(x)
}
*/
#[repr(C)]
pub struct CefRc<T: Is<ffi::cef_base_t>> {
    inner: *mut T
}

pub trait Is<T> {}

impl Is<ffi::cef_base_t> for ffi::cef_base_t {}
impl<T> Is<T> for Void {}
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
    fn make<F: FnOnce(ffi::cef_base_t) -> T>(mut f: F) -> CefRc<T> {
        #[repr(C)]
        struct RefCounted<T> {
            v: T,
            count: libc::c_int
        }
        impl<T> Is<ffi::cef_base_t> for RefCounted<T> {}

        extern fn add_ref<T>(_self: *mut ffi::cef_base_t) {
            let cell: &mut RefCounted<T> = unsafe{ unsafe_downcast_mut(&mut *_self) };
            cell.count += 1;
        }
        extern fn release<T>(_self: *mut ffi::cef_base_t) -> libc::c_int {
            unsafe {
                let cell: *mut RefCounted<T> = transmute(_self);
                (*cell).count -= 1;
                let count = (*cell).count;
                if count == 0 {
                    let cell: Box<RefCounted<T>> = transmute(cell);
                    drop(cell);
                }
                count
            }
        }
        extern fn has_one_ref<T>(_self: *mut ffi::cef_base_t) -> libc::c_int {
            let cell: &mut RefCounted<T> = unsafe{ unsafe_downcast_mut(&mut *_self) };
            if cell.count == 1 { 1 } else { 0 }
        }
        CefRc {
            inner: unsafe { transmute(box RefCounted {
                v: f(ffi::cef_base_t {
                    size: size_of::<RefCounted<T>>() as libc::size_t,
                    add_ref: Some(add_ref::<T>),
                    release: Some(release::<T>),
                    has_one_ref: Some(has_one_ref::<T>)
                }),
                count: 1
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

fn execute_process<T>(app: CefRc<T>) -> libc::c_int
    where T : Is<ffi::cef_app_t> + Is<ffi::cef_base_t>
{
    use std::ffi::CString;
    let args: Vec<CString> = std::os::args().into_iter().map(|x| CString::from_vec(x.into_bytes())).collect();
    let args: Vec<*mut libc::c_char> = args.iter().map(|x| x.as_slice_with_nul().as_ptr() as *mut _).collect();
    let args = &args[];
    let args = ffi::cef_main_args_t { argc: args.len() as libc::c_int, argv: args[].as_ptr() as *mut _ };
    unsafe{ ffi::cef_execute_process(&args as *const _, upcast_ptr(app), zeroed()) }
}

#[repr(C)]
pub struct Settings {
    pub size: ::libc::size_t,
    pub single_process: ::libc::c_int,
    pub no_sandbox: ::libc::c_int,
    pub browser_subprocess_path: CefString,
    pub multi_threaded_message_loop: ::libc::c_int,
    pub windowless_rendering_enabled: ::libc::c_int,
    pub command_line_args_disabled: ::libc::c_int,
    pub cache_path: CefString,
    pub persist_session_cookies: ::libc::c_int,
    pub user_agent: CefString,
    pub product_version: CefString,
    pub locale: CefString,
    pub log_file: CefString,
    pub log_severity: ffi::cef_log_severity_t,
    pub javascript_flags: CefString,
    pub resources_dir_path: CefString,
    pub locales_dir_path: CefString,
    pub pack_loading_disabled: ::libc::c_int,
    pub remote_debugging_port: ::libc::c_int,
    pub uncaught_exception_stack_size: ::libc::c_int,
    pub context_safety_implementation: ::libc::c_int,
    pub ignore_certificate_errors: ::libc::c_int,
    pub background_color: ffi::cef_color_t,
}

impl Settings {
    pub fn new() -> Settings {
        use std::default::Default;

        let mut x: Settings = unsafe { zeroed() };
        x.size = size_of::<ffi::cef_settings_t>() as libc::size_t;
        x.no_sandbox = 1;
        x
    }
    fn settings<'a>(&'a self) -> &'a ffi::cef_settings_t {
        unsafe{ transmute::<&'a Settings, &'a ffi::cef_settings_t>(self) }
    }
}

fn initialize<T>(settings: &Settings, app: CefRc<T>) -> libc::c_int
    where T : Is<ffi::cef_app_t> + Is<ffi::cef_base_t>
{
    use std::ffi::CString;
    let args: Vec<CString> = std::os::args().into_iter().map(|x| CString::from_vec(x.into_bytes())).collect();
    let args: Vec<*mut libc::c_char> = args.iter().map(|x| x.as_slice_with_nul().as_ptr() as *mut _).collect();
    let args = &args[];
    let args = ffi::cef_main_args_t { argc: args.len() as libc::c_int, argv: args[].as_ptr() as *mut _ };
    unsafe{ ffi::cef_initialize(&args as *const _, settings.settings() as *const _, upcast_ptr(app), zeroed()) }
}

#[unsafe_destructor]
impl<T: Is<ffi::cef_base_t>> Drop for CefRc<T> {
    fn drop(&mut self) {
        unsafe{ (*self.inner).release() };
    }
}

impl<T: Is<ffi::cef_base_t>> Clone for CefRc<T> {
    fn clone(&self) -> CefRc<T> {
        unsafe{ (*self.inner).add_ref() };
        CefRc { inner: self.inner }
    }
}

#[test]
fn it_works() {
    let app = App::new(app::DefaultCallback);

    if execute_process(app.clone()) >= 0 {
        return;
    }

    let mut settings = Settings::new();
    settings.log_file = CefString::from_str("log2.log");
    settings.locale = CefString::from_str("en_GB");
    println!("initialising");
    println!("{}", initialize(&settings, app));
    panic!();

}

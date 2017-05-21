/*
    Many build errors here, the code looks overcomplicated and I will not
    deal with this now. TODO: Investigate later.

use ffi;
use CefRc;
use Interface;
use Is;
use browser_host::BrowserHost;
use cast_to_interface;
use cast_from_interface;

#[allow(missing_copy_implementations)]
pub struct Browser {
    vtable: ffi::cef_browser_t,
}

unsafe impl Interface<ffi::cef_browser_t> for Browser {}
unsafe impl Is<ffi::cef_base_ref_counted_t> for Browser {}

impl Browser {
    #[cfg(target_os="windows")]
    fn call0<'a, T>(&'a self,
                    f: &'a Option<extern "stdcall" fn(*mut ffi::cef_browser_t) -> T>)
                    -> T {
        f.as_ref().unwrap()(&self.vtable as *const _ as *mut _)
    }
    #[cfg(not(target_os="windows"))]
    fn call0<'a, T>(&'a self, f: &'a Option<extern "C" fn(*mut ffi::cef_browser_t) -> T>) -> T {
        f.as_ref().unwrap()(&self.vtable as *const _ as *mut _)
    }
    #[cfg(target_os="windows")]
    fn call1<'a, A0, T>(&'a self,
                        f: &'a Option<extern "stdcall" fn(*mut ffi::cef_browser_t, A0) -> T>,
                        a0: A0)
                        -> T {
        f.as_ref().unwrap()(&self.vtable as *const _ as *mut _, a0)
    }
    #[cfg(not(target_os="windows"))]
    fn call1<'a, A0, T>(&'a self,
                        f: &'a Option<extern "C" fn(*mut ffi::cef_browser_t, A0) -> T>,
                        a0: A0)
                        -> T {
        f.as_ref().unwrap()(&self.vtable as *const _ as *mut _, a0)
    }
    pub fn get_host(&self) -> CefRc<BrowserHost> {
        cast_to_interface(self.call0(&self.vtable.get_host))
    }
    pub fn can_go_back(&self) -> bool {
        self.call0(&self.vtable.can_go_back) != 0
    }
    pub fn go_back(&self) {
        self.call0(&self.vtable.go_back)
    }
    pub fn can_go_forward(&self) -> bool {
        self.call0(&self.vtable.can_go_forward) != 0
    }
    pub fn go_forward(&self) {
        self.call0(&self.vtable.go_forward)
    }
    pub fn is_loading(&self) -> bool {
        self.call0(&self.vtable.is_loading) != 0
    }
    pub fn reload(&self) {
        self.call0(&self.vtable.reload)
    }
    pub fn reload_ignore_cache(&self) {
        self.call0(&self.vtable.reload_ignore_cache);
    }
    pub fn stop_load(&self) {
        self.call0(&self.vtable.stop_load)
    }
    pub fn get_identifier(&self) -> i32 {
        self.call0(&self.vtable.get_identifier)
    }
    pub fn is_same(&self, other: CefRc<Browser>) -> bool {
        self.call1(&self.vtable.is_same, cast_from_interface(other)) != 0
    }
    pub fn is_popup(&self) -> bool {
        self.call0(&self.vtable.is_popup) != 0
    }
    pub fn has_document(&self) -> bool {
        self.call0(&self.vtable.has_document) != 0
    }
    //TODO complete
}

*/

use ffi;
use std::slice;
use std::mem::{transmute, replace, forget};
use std::mem;
use alloc::heap::{allocate, deallocate};
use std::ptr::null_mut;
use libc;
use std::ops::{Deref, DerefMut};

trait UTF16Ext {
    fn units<'a>(&'a self) -> &'a [u16];
    fn to_string(&self) -> String;
}

impl UTF16Ext for ffi::cef_string_utf16_t {
    fn units<'a>(&'a self) -> &'a [u16] {
        unsafe {
            slice::from_raw_parts(self._str, self.length as usize)
        }
    }
    fn to_string(&self) -> String {
        String::from_utf16_lossy(self.units())
    }
}

pub trait OwnableString {
    unsafe fn alloc() -> *mut Self;
    unsafe fn free(v: *mut Self);
    fn release(&mut self);
    fn is_drop_fill(&self) -> bool;
}

impl OwnableString for ffi::cef_string_utf16_t {
    unsafe fn alloc() -> *mut Self {
        ffi::cef_string_userfree_utf16_alloc()
    }
    unsafe fn free(v: *mut Self) {
        ffi::cef_string_userfree_utf16_free(v)
    }
    fn release(&mut self) {
        self.dtor.map(|f| f(self._str));
    }
    fn is_drop_fill(&self) -> bool {
        unsafe { transmute::<_, usize>(self._str) == ::std::mem::POST_DROP_USIZE }
    }
}

#[repr(C)]
#[unsafe_no_drop_flag]
pub struct OwnedString<T : OwnableString> {
    v: T
}

impl<T: OwnableString> OwnedString<T> {
    unsafe fn unwrap(mut self) -> T {
        use std::mem::zeroed;
        let out = replace(&mut self.v, zeroed());
        forget(self);
        out
    }
    pub fn view(v: &T) -> &Self {
        unsafe{ transmute(v) }
    }
}
impl<T: OwnableString> Deref for OwnedString<T> {
    type Target = T;
    fn deref<'a>(&'a self) -> &'a T {
        &self.v
    }
}
impl<T : OwnableString> Drop for OwnedString<T> {
    fn drop(&mut self) {
        use std::mem::zeroed;
        if !self.v.is_drop_fill() {
            unsafe{ self.v.release(); self.v = zeroed() }
        }
    }
}

pub type OwnedString16 = OwnedString<ffi::cef_string_utf16_t>;
pub type CefString = OwnedString16;

pub fn cast_from(s: ffi::cef_string_t) -> CefString {
    CefString{ v: s }
}

pub fn cast_from_userfree_ptr(s: ffi::cef_string_userfree_t) -> CefStringUserFreePtr {
    unsafe { transmute(s) }
}

pub fn cast_to_ptr<T: OwnableString>(s: *const OwnedString<T>) -> *const T {
    unsafe{ transmute(s) }
}

#[repr(C)]
#[unsafe_no_drop_flag]
pub struct OwnedStringPtr<T : OwnableString> {
    v: *mut T
}

pub type CefStringUserFreePtr = OwnedStringPtr<ffi::cef_string_utf16_t>;

impl<T: OwnableString> OwnedStringPtr<T> {
    pub fn new(s: OwnedString<T>) -> OwnedStringPtr<T> {
        unsafe {
            let v = T::alloc();
            forget(replace(&mut *v, s.unwrap()));
            OwnedStringPtr { v: v }
        }
    }
}

impl<T: OwnableString> Drop for OwnedStringPtr<T> {
    fn drop(&mut self) {
        use std::mem::zeroed;
        unsafe {
            if transmute::<_, usize>(self.v) != ::std::mem::POST_DROP_USIZE {
                if self.v != null_mut() {
                    (*self.v).release();
                    T::free(self.v);
                }
                self.v = zeroed();
            }
        }
    }
}

impl<T: OwnableString> Deref for OwnedStringPtr<T> {
    type Target = OwnedString<T>;
    fn deref<'a>(&'a self) -> &'a OwnedString<T> {
        unsafe { &(*transmute::<_, *mut _>(self.v)) }
    }
}

impl<T: OwnableString> DerefMut for OwnedStringPtr<T> {
    fn deref_mut<'a>(&'a mut self) -> &'a mut OwnedString<T> {
        unsafe { &mut(*transmute::<_, *mut _>(self.v)) }
    }
}


//pub fn cast_userfree<T : OwnableString>(s: *mut T) -> OwnedStringPtr<T> {
//    OwnedStringPtr { v: s }
//}

impl CefString {
    pub fn cast(self) -> ffi::cef_string_t {
        unsafe { transmute(self) }
    }
    pub fn from_str(s: &str) -> CefString {
        use std::ptr::copy_nonoverlapping;

        let data: Vec<u16> = s.utf16_units().collect();

        let (ptr, size) = if data.len() == 0 {
            (null_mut(), 0)
        } else {
            let size = data.len().checked_mul(mem::size_of::<u16>())
                .and_then(|x| x.checked_add(mem::size_of::<usize>()))
                .expect("capacity overflow");
            let ptr = unsafe { allocate(size, mem::align_of::<usize>()) };
            if ptr.is_null() { ::alloc::oom() }
            (ptr as *mut u16, size)
        };
        let mut ptr = ptr as *mut usize;
        unsafe {
            *ptr = size;
            ptr = ptr.offset(1);
        }
        let ptr = ptr as *mut u16;
        unsafe { copy_nonoverlapping(data.as_ptr(), ptr, data.len()) };
        #[stdcall_win]
        extern fn release(str: *mut u16) {
            if str == null_mut() { return; }
            unsafe {
                let mut ptr = str as *mut usize;
                ptr = ptr.offset(-1);
                let size = *ptr;
                deallocate(ptr as *mut u8, size, mem::align_of::<usize>());
            }
        }

        OwnedString {
            v: ffi::cef_string_utf16_t {
                _str: ptr,
                length: data.len() as libc::size_t,
                dtor: Some(release)
            }
        }
    }
    pub fn units<'a>(&'a self) -> &'a [u16] {
        unsafe {
            slice::from_raw_parts(self.v._str, self.v.length as usize)
        }
    }
    pub fn to_string(&self) -> String {
        String::from_utf16_lossy(self.units())
    }
}

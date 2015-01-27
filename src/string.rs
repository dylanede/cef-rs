use ffi;
use std::slice;
use std::mem::transmute;
use std::mem;
use alloc::heap::{allocate, deallocate};
use std::ptr::null_mut;
use std::num::Int;
use libc;

pub trait UTF16Ext {
    fn units<'a>(&'a self) -> &'a [u16];
    fn to_string(&self) -> String;
}

impl UTF16Ext for ffi::cef_string_utf16_t {
    fn units<'a>(&'a self) -> &'a [u16] {
        unsafe {
            slice::from_raw_buf(transmute::<&'a *mut u16, &'a *const u16>(&self._str), self.length as usize)
        }
    }
    fn to_string(&self) -> String {
        String::from_utf16_lossy(self.units())
    }
}

pub trait OwnableString {
    fn release(&mut self);
}

impl OwnableString for ffi::cef_string_utf16_t {
    fn release(&mut self) {
        self.dtor.map(|f| f(self._str));
    }
}

#[repr(C)]
#[unsafe_no_drop_flag]
pub struct OwnedString<T : OwnableString> {
    v: T
}

#[unsafe_destructor]
impl<T : OwnableString> Drop for OwnedString<T> {
    fn drop(&mut self) {
        use std::mem::zeroed;
        unsafe{ self.v.release(); self.v = zeroed() }
    }
}

pub type OwnedString16 = OwnedString<ffi::cef_string_utf16_t>;
pub type CefString = OwnedString16;

impl CefString {
    pub fn from_str(s: &str) -> OwnedString16 {
        /*
        use std::ffi::CString;
        let cstr = CString::from_vec(string.into_bytes());
        let slice = cstr.as_slice();
        let mut result = unsafe { uninitialized() };
        unsafe {
            ffi::cef_string_utf8_to_utf16(
                slice.as_ptr(),
                slice.len() as libc::size_t,
                &mut result as *mut _);
        }
        OwnedString {
            v: result
        }*/

        use std::ptr::copy_nonoverlapping_memory;

        let data: Vec<u16> = s.utf16_units().collect();

        let (ptr, size) = if data.len() == 0 {
            (null_mut(), 0)
        } else {
            let size = data.len().checked_mul(mem::size_of::<u16>()).and_then(|x| x.checked_add(mem::size_of::<usize>())).expect("capacity overflow");
            let ptr = unsafe { allocate(size, mem::min_align_of::<usize>()) };
            if ptr.is_null() { ::alloc::oom() }
            (ptr as *mut u16, size)
        };
        let mut ptr = ptr as *mut usize;
        unsafe {
            *ptr = size;
            ptr = ptr.offset(1);
        }
        let ptr = ptr as *mut u16;
        unsafe { copy_nonoverlapping_memory(ptr, data.as_ptr(), data.len()) };
        extern fn release(str: *mut u16) {
            if str == null_mut() { return; }
            unsafe {
                let mut ptr = str as *mut usize;
                ptr = ptr.offset(-1);
                let size = *ptr;
                deallocate(ptr as *mut u8, size, mem::min_align_of::<usize>());
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
}

use ffi;
use CefRc;
use Interface;
use RefCountable;
use cast_to_interface;
use cast_from_interface;
use ::std::borrow::Cow;
use ::string::CefString;

#[allow(missing_copy_implementations)]
pub struct ListValue {
    vtable: ffi::cef_list_value_t
}

unsafe impl Interface<ffi::cef_list_value_t> for ListValue {}
unsafe impl RefCountable for ListValue {}

impl ListValue {
    #[cfg(target_os="windows")]
    fn call0<'a, T>(&'a self, f: &'a Option<extern "stdcall" fn(*mut ffi::cef_list_value_t) -> T>) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _)
    }
    #[cfg(not(target_os="windows"))]
    fn call0<'a, T>(&'a self, f: &'a Option<extern "C" fn(*mut ffi::cef_list_value_t) -> T>) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _)
    }
    #[cfg(target_os="windows")]
    fn call1<'a, A0, T>(&'a self,
        f: &'a Option<extern "stdcall" fn(*mut ffi::cef_list_value_t, A0) -> T>,
        a0: A0) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _, a0)
    }
    #[cfg(not(target_os="windows"))]
    fn call1<'a, A0, T>(
        &'a self,
        f: &'a Option<extern "C" fn(*mut ffi::cef_list_value_t, A0) -> T>,
        a0: A0) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _, a0)
    }
    #[cfg(target_os="windows")]
    fn call2<'a, A0, A1, T>(&'a self,
        f: &'a Option<extern "stdcall" fn(*mut ffi::cef_list_value_t, A0, A1) -> T>,
        a0: A0, a1: A1) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _, a0, a1)
    }
    #[cfg(not(target_os="windows"))]
    fn call2<'a, A0, A1, T>(
        &'a self,
        f: &'a Option<extern "C" fn(*mut ffi::cef_list_value_t, A0, A1) -> T>,
        a0: A0, a1: A1) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _, a0, a1)
    }
    pub fn clear(&self) -> bool {
        self.call0(&self.vtable.clear) != 0
    }
    pub fn copy(&self) -> CefRc<ListValue> {
        cast_to_interface(self.call0(&self.vtable.copy))
    }
    pub fn new() -> CefRc<ListValue> {
        unsafe{ cast_to_interface(ffi::cef_list_value_create()) }
    }
    pub fn from_vec(v: Vec<ValueItem>) -> CefRc<ListValue> {
        let l = ListValue::new();
        assert!(l.set_len(v.len()));
        for (i, item) in v.into_iter().enumerate() {
            assert!(l.set(i, item));
        }
        l
    }
    pub fn len(&self) -> usize {
        self.call0(&self.vtable.get_size) as usize
    }
    pub fn set_len(&self, len: usize) -> bool {
        self.call1(&self.vtable.set_size, len as ::libc::size_t) != 0
    }
    pub fn get(&self, index: usize) -> Option<ValueItem<'static>> {
        if self.len() <= index {
            None
        } else {
            let index = index as ::libc::c_int;
            match self.call1(&self.vtable.get_type, index) {
                ffi::VTYPE_BINARY => Some(ValueItem::Binary(
                    cast_to_interface(self.call1(&self.vtable.get_binary, index)))),
                ffi::VTYPE_BOOL => Some(ValueItem::Bool(self.call1(&self.vtable.get_bool, index) != 0)),
                ffi::VTYPE_DICTIONARY => Some(ValueItem::Dictionary(
                    cast_to_interface(self.call1(&self.vtable.get_dictionary, index)))),
                ffi::VTYPE_DOUBLE => Some(ValueItem::Double(self.call1(&self.vtable.get_double, index))),
                ffi::VTYPE_INT => Some(ValueItem::Int(self.call1(&self.vtable.get_int, index))),
                ffi::VTYPE_LIST => Some(ValueItem::List(
                    cast_to_interface(self.call1(&self.vtable.get_list, index)))),
                ffi::VTYPE_STRING => {
                    let cef_str_ptr = ::string::cast_from_userfree_ptr(
                        self.call1(&self.vtable.get_string, index));
                    Some(ValueItem::String(Cow::Owned(cef_str_ptr.to_string())))
                },
                ffi::VTYPE_NULL => Some(ValueItem::Null),
                ffi::VTYPE_INVALID => None,
                _ => unreachable!()
            }
        }
    }
    pub fn set(&self, index: usize, item: ValueItem) -> bool {
        let index = index as ::libc::c_int;
        match item {
            ValueItem::Binary(bin) => self.call2(&self.vtable.set_binary, index, cast_from_interface(bin)) != 0,
            ValueItem::Bool(b) => self.call2(&self.vtable.set_bool, index, b as ::libc::c_int) != 0,
            ValueItem::Dictionary(dic) => self.call2(&self.vtable.set_dictionary,
                                                    index,
                                                    cast_from_interface(dic)) != 0,
            ValueItem::Double(d) => self.call2(&self.vtable.set_double, index, d) != 0,
            ValueItem::Int(i) => self.call2(&self.vtable.set_int, index, i as ::libc::c_int) != 0,
            ValueItem::List(l) => self.call2(&self.vtable.set_list, index, cast_from_interface(l)) != 0,
            ValueItem::String(s) => {
                let cef_str = CefString::from_str(&s);
                self.call2(&self.vtable.set_string, index, &*cef_str as *const _) != 0
            }
            ValueItem::Null => self.call1(&self.vtable.set_null, index) != 0,
        }
    }
    pub fn remove(&self, index: usize) -> bool {
        self.call1(&self.vtable.remove, index as ::libc::c_int) != 0
    }
    pub fn is_valid(&self) -> bool {
        self.call0(&self.vtable.is_valid) != 0
    }
    pub fn is_owned(&self) -> bool {
        self.call0(&self.vtable.is_owned) != 0
    }
    pub fn is_read_only(&self) -> bool {
        self.call0(&self.vtable.is_read_only) != 0
    }
}

pub enum ValueItem<'a> {
    Binary(CefRc<BinaryValue>),
    Bool(bool),
    Dictionary(CefRc<DictionaryValue>),
    Double(f64),
    Int(i32),
    List(CefRc<ListValue>),
    String(Cow<'a, str>),
    Null
}

#[allow(missing_copy_implementations)]
pub struct BinaryValue {
    vtable: ffi::cef_binary_value_t
}

unsafe impl Interface<ffi::cef_binary_value_t> for BinaryValue {}
unsafe impl RefCountable for BinaryValue {}
fn garbage_vec(len: usize) -> Vec<u8> {
    let mut v = Vec::with_capacity(len);
    unsafe{ v.set_len(len) }
    v
}
impl BinaryValue {
    #[cfg(target_os="windows")]
    fn call0<'a, T>(&'a self, f: &'a Option<extern "stdcall" fn(*mut ffi::cef_binary_value_t) -> T>) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _)
    }
    #[cfg(not(target_os="windows"))]
    fn call0<'a, T>(&'a self, f: &'a Option<extern "C" fn(*mut ffi::cef_binary_value_t) -> T>) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _)
    }
    pub fn new(data: &[u8]) -> CefRc<BinaryValue> {
        unimplemented!()
    }
    pub fn copy(&self) -> CefRc<BinaryValue> {
        cast_to_interface(self.call0(&self.vtable.copy))
    }
    pub fn get_data(&self, offset: usize, len: usize) -> Option<Box<[u8]>> {
        use ::libc::size_t;
        use libc::c_void;
        if let Some(true) = offset.checked_add(len).map(|end| end <= self.len()) {
            let mut buffer = garbage_vec(len).into_boxed_slice();
            let read = self.vtable.get_data.as_ref().unwrap()(
                &self.vtable as * const _ as *mut _,
                buffer.as_mut_ptr() as *mut c_void,
                len as size_t,
                offset as size_t) as usize;
            if read == len {
                Some(buffer)
            } else {
                None
            }
        } else {
            None
        }
    }
    pub fn get_all_data(&self) -> Option<Box<[u8]>> {
        use ::libc::size_t;
        use libc::c_void;
        let mut buffer = garbage_vec(self.len()).into_boxed_slice();
        let read = self.vtable.get_data.as_ref().unwrap()(
            &self.vtable as * const _ as *mut _,
            buffer.as_mut_ptr() as *mut c_void,
            buffer.len() as size_t,
            0) as usize;
        if read == buffer.len() {
            Some(buffer)
        } else {
            None
        }
    }
    pub fn len(&self) -> usize {
        self.call0(&self.vtable.get_size) as usize
    }
    pub fn is_valid(&self) -> bool {
        self.call0(&self.vtable.is_valid) != 0
    }
    pub fn is_owned(&self) -> bool {
        self.call0(&self.vtable.is_owned) != 0
    }
}

#[allow(missing_copy_implementations)]
pub struct DictionaryValue {
    vtable: ffi::cef_dictionary_value_t
}

unsafe impl Interface<ffi::cef_dictionary_value_t> for DictionaryValue {}
unsafe impl RefCountable for DictionaryValue {}

impl DictionaryValue {
    #[cfg(target_os="windows")]
    fn call0<'a, T>(&'a self, f: &'a Option<extern "stdcall" fn(*mut ffi::cef_dictionary_value_t) -> T>) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _)
    }
    #[cfg(not(target_os="windows"))]
    fn call0<'a, T>(&'a self, f: &'a Option<extern "C" fn(*mut ffi::cef_dictionary_value_t) -> T>) -> T {
        f.as_ref().unwrap()(&self.vtable as * const _ as *mut _)
    }
    pub fn len(&self) -> usize {
        self.call0(&self.vtable.get_size) as usize
    }
    pub fn is_valid(&self) -> bool {
        self.call0(&self.vtable.is_valid) != 0
    }
    pub fn is_owned(&self) -> bool {
        self.call0(&self.vtable.is_owned) != 0
    }
    pub fn is_read_only(&self) -> bool {
        self.call0(&self.vtable.is_read_only) != 0
    }
}

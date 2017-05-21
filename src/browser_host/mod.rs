use ffi;
use libc;
use BrowserClientWrapper;
use Browser;
use BrowserClient;
use WindowInfo;
use CefString;
use CefRc;
use State;
use upcast_ptr;
use std::mem::{transmute, zeroed};
use string;
use cast_to_interface;
use upcast;

use Interface;
use Is;

use std::ptr::null_mut;
use std::default::Default;

mod keys;

#[derive(Debug, Copy, Clone)]
pub struct Modifiers {
    pub shift: bool,
    pub control: bool,
    pub alt: bool,
}

impl Modifiers {
    pub fn new() -> Modifiers {
        Modifiers {
            shift: false,
            control: false,
            alt: false,
        }
    }
}

pub mod event_flags {
    bitflags! {
        flags EventFlags: u32 {
            const NONE = 0,
            const CAPS_LOCK_ON = 1,
            const SHIFT_DOWN = 2,
            const CONTROL_DOWN = 4,
            const ALT_DOWN = 8,
            const LEFT_MOUSE_BUTTON = 16,
            const MIDDLE_MOUSE_BUTTON = 32,
            const RIGHT_MOUSE_BUTTON = 64,
            const COMMAND_DOWN = 128,
            const NUM_LOCK_ON = 256,
            const IS_KEY_PAD = 512,
            const IS_LEFT = 1024,
            const IS_RIGHT = 2048
        }
    }
}
pub use self::event_flags::EventFlags;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct MouseEvent {
    pub x: i32,
    pub y: i32,
    pub modifiers: EventFlags,
}

impl Default for MouseEvent {
    fn default() -> MouseEvent {
        MouseEvent {
            modifiers: EventFlags::empty(),
            x: 0,
            y: 0,
        }
    }
}

unsafe impl Is<ffi::cef_mouse_event_t> for MouseEvent {}

#[test]
fn check_mouse_event_size() {
    use std::mem::size_of;
    assert!(size_of::<MouseEvent>() == size_of::<ffi::cef_mouse_event_t>());
}

#[derive(Copy, Clone)]
#[repr(u32)]
pub enum MouseButtonType {
    Left,
    Middle,
    Right,
}

#[allow(missing_copy_implementations)]
pub struct BrowserHost {
    vtable: ffi::cef_browser_host_t,
}

unsafe impl Interface<ffi::cef_browser_host_t> for BrowserHost {}
unsafe impl Is<ffi::cef_base_t> for BrowserHost {}

/// TODO: Investigate and implement.
pub enum RequestContext {}

impl BrowserHost {
    #[cfg(target_os="windows")]
    fn call0<'a, T>(&'a self,
                    f: &'a Option<extern "stdcall" fn(*mut ffi::cef_browser_host_t) -> T>)
                    -> T {
        f.as_ref().unwrap()(&self.vtable as *const _ as *mut _)
    }
    #[cfg(not(target_os="windows"))]
    fn call0<'a, T>(&'a self,
                    f: &'a Option<extern "C" fn(*mut ffi::cef_browser_host_t) -> T>)
                    -> T {
        f.as_ref().unwrap()(&self.vtable as *const _ as *mut _)
    }
    #[cfg(target_os="windows")]
    fn call1<'a, A0, T>(&'a self,
                        f: &'a Option<extern "stdcall" fn(*mut ffi::cef_browser_host_t, A0) -> T>,
                        a0: A0)
                        -> T {
        f.as_ref().unwrap()(&self.vtable as *const _ as *mut _, a0)
    }
    #[cfg(not(target_os="windows"))]
    fn call1<'a, A0, T>(&'a self,
                        f: &'a Option<extern "C" fn(*mut ffi::cef_browser_host_t, A0) -> T>,
                        a0: A0)
                        -> T {
        f.as_ref().unwrap()(&self.vtable as *const _ as *mut _, a0)
    }

    pub fn was_resized(&self) {
        self.call0(&self.vtable.was_resized)
    }

    pub fn set_focus(&self, focused: bool) {
        self.call1(&self.vtable.send_focus_event, focused as libc::c_int);
    }

    pub fn close_browser(&self, force: bool) {
        self.call1(&self.vtable.close_browser, force as libc::c_int)
    }

    pub fn send_mouse_click_event(&self,
                                  event: &MouseEvent,
                                  button: MouseButtonType,
                                  mouse_up: bool,
                                  click_count: i32) {
        unsafe {
            self.vtable.send_mouse_click_event.as_ref().unwrap()(&self.vtable as *const _ as *mut _,
                                                                 upcast(event) as *const _,
                                                                 transmute(button),
                                                                 mouse_up as i32,
                                                                 click_count)
        }
    }

    pub fn send_mouse_move_event(&self, event: &MouseEvent, mouse_leave: bool) {
        self.vtable.send_mouse_move_event.as_ref().unwrap()(&self.vtable as *const _ as *mut _,
                                                            upcast(event) as *const _,
                                                            mouse_leave as i32)
    }

    pub fn send_mouse_wheel_event(&self, event: &MouseEvent, delta: (i32, i32)) {
        self.vtable.send_mouse_wheel_event.as_ref().unwrap()(&self.vtable as *const _ as *mut _,
                                                             upcast(event) as *const _,
                                                             delta.0,
                                                             delta.1)
    }

    pub fn send_char(&self, c: char) {
        println!("CHAR EVENT {}", c);
        let modifiers = keys::modifiers_for_char(c);
        println!("char modifiers: {:?}", modifiers);
        //let keycode = keys::win_vk_for_char(c);
        let scan_code = keys::scan_code_for_char(c);
        let c = c as u8;
        //println!("sending key code: {:?}", keycode);
        //let keycode = unsafe{ transmute(keycode) };
        let mut mods = 0;
        if modifiers.shift {
            mods |= ffi::EVENTFLAG_SHIFT_DOWN;
        }
        if modifiers.alt {
            mods |= ffi::EVENTFLAG_ALT_DOWN;
        }
        if modifiers.control {
            mods |= ffi::EVENTFLAG_CONTROL_DOWN;
        }
        let native_key_code: i32 = if cfg!(target_os = "windows") {
            (((scan_code as u32) << 16) | 1) as i32
        } else {
            unimplemented!()
        };
        let event = ffi::cef_key_event_t {
            _type: ffi::KEYEVENT_CHAR,
            modifiers: mods as u32,
            windows_key_code: c as i32,
            native_key_code: native_key_code,
            is_system_key: 0,
            character: c as u16,
            unmodified_character: c as u16,
            focus_on_editable_field: 0,
        };
        self.call1(&self.vtable.send_key_event, &event as *const _)
    }
    pub fn send_key_event(&self, pressed: bool, scan_code: u8, modifiers: Modifiers) {
        println!("KEY EVENT");
        let keycode = keys::win_vk_for_scan_code(scan_code);
        let c = keys::char_for_scan_code(scan_code);
        println!("sending modifiers: {:?}", modifiers);
        println!("sending char: {:?}", c as char);
        println!("sending key code: {:?}", keycode);
        let mut mods = 0;
        if modifiers.shift {
            mods |= ffi::EVENTFLAG_SHIFT_DOWN;
        }
        if modifiers.alt {
            mods |= ffi::EVENTFLAG_ALT_DOWN;
        }
        if modifiers.control {
            mods |= ffi::EVENTFLAG_CONTROL_DOWN;
        }
        let keycode = unsafe { transmute(keycode) };
        let native_key_code: i32 = if cfg!(target_os = "windows") {
            (((scan_code as u32) << 16) | 1) as i32
        } else {
            unimplemented!()
        };
        let mut event = ffi::cef_key_event_t {
            _type: if pressed {
                ffi::KEYEVENT_KEYDOWN
            } else {
                ffi::KEYEVENT_KEYUP
            },
            modifiers: mods as u32,
            windows_key_code: keycode,
            native_key_code: native_key_code,
            is_system_key: 0,
            character: c as u16,
            unmodified_character: c as u16,
            focus_on_editable_field: 0,
        };
        if cfg!(target_os = "windows") && !pressed {
            event.native_key_code |= 0xC0000000u32 as i32;
        }
        self.call1(&self.vtable.send_key_event, &event as *const _)
    }

    /// TODO: Review and implement request_context handling.
    #[allow(unused_variables)]
    pub fn create_browser_sync<T: BrowserClient>(window_info: &WindowInfo,
                                                 client: T,
                                                 url: &str,
                                                 settings: &BrowserSettings,
                                                 request_context: Option<RequestContext>)
                                                 -> CefRc<Browser> {
        let info = window_info.to_cef();
        let url = CefString::from_str(url);
        unsafe {
            assert!(ffi::cef_currently_on(ffi::TID_UI) == 1);
            let ptr = ffi::cef_browser_host_create_browser_sync(
                &info as *const _,
                upcast_ptr(BrowserClientWrapper::new(client)),
                string::cast_to_ptr(&url as *const _),
                settings.settings() as *const _,
                zeroed());
            drop(info);
            drop(url);
            if ptr == null_mut() {
                panic!("Generated browser is null!");
            }
            cast_to_interface(ptr)
        }
    }

    /// TODO: Review and implement request_context handling.
    #[allow(unused_variables)]
    pub fn create_browser<T: BrowserClient + Send>(window_info: &WindowInfo,
                                                   client: T,
                                                   url: &str,
                                                   settings: &BrowserSettings,
                                                   request_context: Option<RequestContext>)
                                                   -> bool {
        let info = window_info.to_cef();
        let url = CefString::from_str(url);
        let result = unsafe {
            ffi::cef_browser_host_create_browser(&info as *const _,
                                                 upcast_ptr(BrowserClientWrapper::new(client)),
                                                 string::cast_to_ptr(&url as *const _),
                                                 settings.settings() as *const _,
                                                 zeroed()) != 0
        };
        drop(url);
        drop(info);
        result
    }
}

#[repr(C)]
pub struct BrowserSettings {
    pub size: libc::size_t,
    pub windowless_frame_rate: ::libc::c_int,
    pub standard_font_family: CefString,
    pub fixed_font_family: CefString,
    pub serif_font_family: CefString,
    pub sans_serif_font_family: CefString,
    pub cursive_font_family: CefString,
    pub fantasy_font_family: CefString,
    pub default_font_size: ::libc::c_int,
    pub default_fixed_font_size: ::libc::c_int,
    pub minimum_font_size: ::libc::c_int,
    pub minimum_logical_font_size: ::libc::c_int,
    pub default_encoding: CefString,
    pub remote_fonts: State,
    pub javascript: State,
    pub javascript_open_windows: State,
    pub javascript_close_windows: State,
    pub javascript_access_clipboard: State,
    pub javascript_dom_paste: State,
    pub caret_browsing: State,
    pub java: State,
    pub plugins: State,
    pub universal_access_from_file_urls: State,
    pub file_access_from_file_urls: State,
    pub web_security: State,
    pub image_loading: State,
    pub image_shrink_standalone_to_fit: State,
    pub text_area_resize: State,
    pub tab_to_links: State,
    pub local_storage: State,
    pub databases: State,
    pub application_cache: State,
    pub webgl: State,
    pub background_color: ffi::cef_color_t,
}

impl BrowserSettings {
    pub fn new() -> BrowserSettings {
        use std::mem::{zeroed, size_of};
        let mut x: BrowserSettings = unsafe { zeroed() };
        x.size = size_of::<BrowserSettings>() as libc::size_t;
        x
    }
    fn settings<'a>(&'a self) -> &'a ffi::cef_browser_settings_t {
        unsafe { transmute(self) }
    }
}

#[test]
fn check_browser_settings_size() {
    use std::mem::size_of;
    assert!(size_of::<BrowserSettings>() == size_of::<ffi::cef_browser_settings_t>());
}

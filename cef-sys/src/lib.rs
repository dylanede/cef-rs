#![feature(libc, negate_unsigned)]
#![allow(non_camel_case_types, non_snake_case, raw_pointer_derive, missing_copy_implementations, improper_ctypes)]
extern crate libc;

#[cfg(target_os="linux")]
mod linux;
#[cfg(target_os="linux")]
pub use linux::*;

#[cfg(target_os="macos")]
mod mac;
#[cfg(target_os="macos")]
pub use mac::*;

#[cfg(target_os="windows")]
mod windows;
#[cfg(target_os="windows")]
pub use windows::*;

use libc::size_t;
use libc::wchar_t;
use libc::time_t;

#[cfg(target_os="macos")]
#[link(name = "Chromium Embedded Framework", kind = "framework")]
extern "C" {}

#[cfg(target_os="windows")]
#[link(name = "libcef")]
extern {}

#[cfg(target_os="linux")]
#[link(name = "cef")]
extern {}

extern "C" {
    pub fn cef_string_wide_set(src: *const wchar_t, src_len: size_t,
                               output: *mut cef_string_wide_t,
                               copy: ::libc::c_int) -> ::libc::c_int;
    pub fn cef_string_utf8_set(src: *const ::libc::c_char, src_len: size_t,
                               output: *mut cef_string_utf8_t,
                               copy: ::libc::c_int) -> ::libc::c_int;
    pub fn cef_string_utf16_set(src: *const char16, src_len: size_t,
                                output: *mut cef_string_utf16_t,
                                copy: ::libc::c_int) -> ::libc::c_int;
    pub fn cef_string_wide_clear(str: *mut cef_string_wide_t) -> ();
    pub fn cef_string_utf8_clear(str: *mut cef_string_utf8_t) -> ();
    pub fn cef_string_utf16_clear(str: *mut cef_string_utf16_t) -> ();
    pub fn cef_string_wide_cmp(str1: *const cef_string_wide_t,
                               str2: *const cef_string_wide_t)
     -> ::libc::c_int;
    pub fn cef_string_utf8_cmp(str1: *const cef_string_utf8_t,
                               str2: *const cef_string_utf8_t)
     -> ::libc::c_int;
    pub fn cef_string_utf16_cmp(str1: *const cef_string_utf16_t,
                                str2: *const cef_string_utf16_t)
     -> ::libc::c_int;
    pub fn cef_string_wide_to_utf8(src: *const wchar_t, src_len: size_t,
                                   output: *mut cef_string_utf8_t)
     -> ::libc::c_int;
    pub fn cef_string_utf8_to_wide(src: *const ::libc::c_char,
                                   src_len: size_t,
                                   output: *mut cef_string_wide_t)
     -> ::libc::c_int;
    pub fn cef_string_wide_to_utf16(src: *const wchar_t, src_len: size_t,
                                    output: *mut cef_string_utf16_t)
     -> ::libc::c_int;
    pub fn cef_string_utf16_to_wide(src: *const char16, src_len: size_t,
                                    output: *mut cef_string_wide_t)
     -> ::libc::c_int;
    pub fn cef_string_utf8_to_utf16(src: *const ::libc::c_char,
                                    src_len: size_t,
                                    output: *mut cef_string_utf16_t)
     -> ::libc::c_int;
    pub fn cef_string_utf16_to_utf8(src: *const char16, src_len: size_t,
                                    output: *mut cef_string_utf8_t)
     -> ::libc::c_int;
    pub fn cef_string_ascii_to_wide(src: *const ::libc::c_char,
                                    src_len: size_t,
                                    output: *mut cef_string_wide_t)
     -> ::libc::c_int;
    pub fn cef_string_ascii_to_utf16(src: *const ::libc::c_char,
                                     src_len: size_t,
                                     output: *mut cef_string_utf16_t)
     -> ::libc::c_int;
    pub fn cef_string_userfree_wide_alloc() -> cef_string_userfree_wide_t;
    pub fn cef_string_userfree_utf8_alloc() -> cef_string_userfree_utf8_t;
    pub fn cef_string_userfree_utf16_alloc() -> cef_string_userfree_utf16_t;
    pub fn cef_string_userfree_wide_free(str: cef_string_userfree_wide_t)
     -> ();
    pub fn cef_string_userfree_utf8_free(str: cef_string_userfree_utf8_t)
     -> ();
    pub fn cef_string_userfree_utf16_free(str: cef_string_userfree_utf16_t)
     -> ();
    pub fn cef_string_list_alloc() -> cef_string_list_t;
    pub fn cef_string_list_size(list: cef_string_list_t) -> ::libc::c_int;
    pub fn cef_string_list_value(list: cef_string_list_t,
                                 index: ::libc::c_int,
                                 value: *mut cef_string_t) -> ::libc::c_int;
    pub fn cef_string_list_append(list: cef_string_list_t,
                                  value: *const cef_string_t) -> ();
    pub fn cef_string_list_clear(list: cef_string_list_t) -> ();
    pub fn cef_string_list_free(list: cef_string_list_t) -> ();
    pub fn cef_string_list_copy(list: cef_string_list_t) -> cef_string_list_t;
    pub fn cef_string_map_alloc() -> cef_string_map_t;
    pub fn cef_string_map_size(map: cef_string_map_t) -> ::libc::c_int;
    pub fn cef_string_map_find(map: cef_string_map_t,
                               key: *const cef_string_t,
                               value: *mut cef_string_t) -> ::libc::c_int;
    pub fn cef_string_map_key(map: cef_string_map_t, index: ::libc::c_int,
                              key: *mut cef_string_t) -> ::libc::c_int;
    pub fn cef_string_map_value(map: cef_string_map_t, index: ::libc::c_int,
                                value: *mut cef_string_t) -> ::libc::c_int;
    pub fn cef_string_map_append(map: cef_string_map_t,
                                 key: *const cef_string_t,
                                 value: *const cef_string_t) -> ::libc::c_int;
    pub fn cef_string_map_clear(map: cef_string_map_t) -> ();
    pub fn cef_string_map_free(map: cef_string_map_t) -> ();
    pub fn cef_string_multimap_alloc() -> cef_string_multimap_t;
    pub fn cef_string_multimap_size(map: cef_string_multimap_t)
     -> ::libc::c_int;
    pub fn cef_string_multimap_find_count(map: cef_string_multimap_t,
                                          key: *const cef_string_t)
     -> ::libc::c_int;
    pub fn cef_string_multimap_enumerate(map: cef_string_multimap_t,
                                         key: *const cef_string_t,
                                         value_index: ::libc::c_int,
                                         value: *mut cef_string_t)
     -> ::libc::c_int;
    pub fn cef_string_multimap_key(map: cef_string_multimap_t,
                                   index: ::libc::c_int,
                                   key: *mut cef_string_t) -> ::libc::c_int;
    pub fn cef_string_multimap_value(map: cef_string_multimap_t,
                                     index: ::libc::c_int,
                                     value: *mut cef_string_t)
     -> ::libc::c_int;
    pub fn cef_string_multimap_append(map: cef_string_multimap_t,
                                      key: *const cef_string_t,
                                      value: *const cef_string_t)
     -> ::libc::c_int;
    pub fn cef_string_multimap_clear(map: cef_string_multimap_t) -> ();
    pub fn cef_string_multimap_free(map: cef_string_multimap_t) -> ();
    pub fn cef_time_to_timet(cef_time: *const cef_time_t, time: *mut time_t)
     -> ::libc::c_int;
    pub fn cef_time_from_timet(time: time_t, cef_time: *mut cef_time_t)
     -> ::libc::c_int;
    pub fn cef_time_to_doublet(cef_time: *const cef_time_t,
                               time: *mut ::libc::c_double) -> ::libc::c_int;
    pub fn cef_time_from_doublet(time: ::libc::c_double,
                                 cef_time: *mut cef_time_t) -> ::libc::c_int;
    pub fn cef_time_now(cef_time: *mut cef_time_t) -> ::libc::c_int;
    pub fn cef_time_delta(cef_time1: *const cef_time_t,
                          cef_time2: *const cef_time_t,
                          delta: *mut ::libc::c_longlong) -> ::libc::c_int;
    pub fn cef_command_line_create() -> *mut cef_command_line_t;
    pub fn cef_command_line_get_global() -> *mut cef_command_line_t;
    pub fn cef_stream_reader_create_for_file(fileName: *const cef_string_t)
     -> *mut cef_stream_reader_t;
    pub fn cef_stream_reader_create_for_data(data: *mut ::libc::c_void,
                                             size: size_t)
     -> *mut cef_stream_reader_t;
    pub fn cef_stream_reader_create_for_handler(handler:
                                                    *mut cef_read_handler_t)
     -> *mut cef_stream_reader_t;
    pub fn cef_stream_writer_create_for_file(fileName: *const cef_string_t)
     -> *mut cef_stream_writer_t;
    pub fn cef_stream_writer_create_for_handler(handler:
                                                    *mut cef_write_handler_t)
     -> *mut cef_stream_writer_t;
    pub fn cef_drag_data_create() -> *mut cef_drag_data_t;
    pub fn cef_request_create() -> *mut cef_request_t;
    pub fn cef_post_data_create() -> *mut cef_post_data_t;
    pub fn cef_post_data_element_create() -> *mut cef_post_data_element_t;
    pub fn cef_binary_value_create(data: *const ::libc::c_void,
                                   data_size: size_t)
     -> *mut cef_binary_value_t;
    pub fn cef_dictionary_value_create() -> *mut cef_dictionary_value_t;
    pub fn cef_list_value_create() -> *mut cef_list_value_t;
    pub fn cef_process_message_create(name: *const cef_string_t)
     -> *mut cef_process_message_t;
    pub fn cef_cookie_manager_get_global_manager()
     -> *mut cef_cookie_manager_t;
    pub fn cef_cookie_manager_create_manager(path: *const cef_string_t,
                                             persist_session_cookies:
                                                 ::libc::c_int)
     -> *mut cef_cookie_manager_t;
    pub fn cef_request_context_get_global_context()
     -> *mut cef_request_context_t;
    pub fn cef_request_context_create_context(handler:
                                                  *mut Struct__cef_request_context_handler_t)
     -> *mut cef_request_context_t;
    pub fn cef_browser_host_create_browser(windowInfo:
                                               *const cef_window_info_t,
                                           client: *mut Struct__cef_client_t,
                                           url: *const cef_string_t,
                                           settings:
                                               *const Struct__cef_browser_settings_t,
                                           request_context:
                                               *mut Struct__cef_request_context_t)
     -> ::libc::c_int;
    pub fn cef_browser_host_create_browser_sync(windowInfo:
                                                    *const cef_window_info_t,
                                                client:
                                                    *mut Struct__cef_client_t,
                                                url: *const cef_string_t,
                                                settings:
                                                    *const Struct__cef_browser_settings_t,
                                                request_context:
                                                    *mut Struct__cef_request_context_t)
     -> *mut cef_browser_t;
    pub fn cef_print_settings_create() -> *mut cef_print_settings_t;
    pub fn cef_task_runner_get_for_current_thread() -> *mut cef_task_runner_t;
    pub fn cef_task_runner_get_for_thread(threadId: cef_thread_id_t)
     -> *mut cef_task_runner_t;
    pub fn cef_currently_on(threadId: cef_thread_id_t) -> ::libc::c_int;
    pub fn cef_post_task(threadId: cef_thread_id_t, task: *mut cef_task_t)
     -> ::libc::c_int;
    pub fn cef_post_delayed_task(threadId: cef_thread_id_t,
                                 task: *mut cef_task_t, delay_ms: int64)
     -> ::libc::c_int;
    pub fn cef_v8context_get_current_context() -> *mut cef_v8context_t;
    pub fn cef_v8context_get_entered_context() -> *mut cef_v8context_t;
    pub fn cef_v8context_in_context() -> ::libc::c_int;
    pub fn cef_v8value_create_undefined() -> *mut cef_v8value_t;
    pub fn cef_v8value_create_null() -> *mut cef_v8value_t;
    pub fn cef_v8value_create_bool(value: ::libc::c_int)
     -> *mut cef_v8value_t;
    pub fn cef_v8value_create_int(value: int32) -> *mut cef_v8value_t;
    pub fn cef_v8value_create_uint(value: uint32) -> *mut cef_v8value_t;
    pub fn cef_v8value_create_double(value: ::libc::c_double)
     -> *mut cef_v8value_t;
    pub fn cef_v8value_create_date(date: *const cef_time_t)
     -> *mut cef_v8value_t;
    pub fn cef_v8value_create_string(value: *const cef_string_t)
     -> *mut cef_v8value_t;
    pub fn cef_v8value_create_object(accessor: *mut cef_v8accessor_t)
     -> *mut cef_v8value_t;
    pub fn cef_v8value_create_array(length: ::libc::c_int)
     -> *mut cef_v8value_t;
    pub fn cef_v8value_create_function(name: *const cef_string_t,
                                       handler: *mut cef_v8handler_t)
     -> *mut cef_v8value_t;
    pub fn cef_v8stack_trace_get_current(frame_limit: ::libc::c_int)
     -> *mut cef_v8stack_trace_t;
    pub fn cef_register_extension(extension_name: *const cef_string_t,
                                  javascript_code: *const cef_string_t,
                                  handler: *mut cef_v8handler_t)
     -> ::libc::c_int;
    pub fn cef_response_create() -> *mut cef_response_t;
    pub fn cef_register_scheme_handler_factory(scheme_name:
                                                   *const cef_string_t,
                                               domain_name:
                                                   *const cef_string_t,
                                               factory:
                                                   *mut cef_scheme_handler_factory_t)
     -> ::libc::c_int;
    pub fn cef_clear_scheme_handler_factories() -> ::libc::c_int;
    pub fn cef_execute_process(args: *const Struct__cef_main_args_t,
                               application: *mut cef_app_t,
                               windows_sandbox_info: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cef_initialize(args: *const Struct__cef_main_args_t,
                          settings: *const Struct__cef_settings_t,
                          application: *mut cef_app_t,
                          windows_sandbox_info: *mut ::libc::c_void)
     -> ::libc::c_int;
    pub fn cef_shutdown() -> ();
    pub fn cef_do_message_loop_work() -> ();
    pub fn cef_run_message_loop() -> ();
    pub fn cef_quit_message_loop() -> ();
    pub fn cef_set_osmodal_loop(osModalLoop: ::libc::c_int) -> ();
    pub fn cef_visit_web_plugin_info(visitor:
                                         *mut cef_web_plugin_info_visitor_t)
     -> ();
    pub fn cef_refresh_web_plugins() -> ();
    pub fn cef_add_web_plugin_path(path: *const cef_string_t) -> ();
    pub fn cef_add_web_plugin_directory(dir: *const cef_string_t) -> ();
    pub fn cef_remove_web_plugin_path(path: *const cef_string_t) -> ();
    pub fn cef_unregister_internal_web_plugin(path: *const cef_string_t)
     -> ();
    pub fn cef_force_web_plugin_shutdown(path: *const cef_string_t) -> ();
    pub fn cef_register_web_plugin_crash(path: *const cef_string_t) -> ();
    pub fn cef_is_web_plugin_unstable(path: *const cef_string_t,
                                      callback:
                                          *mut cef_web_plugin_unstable_callback_t)
     -> ();
    pub fn cef_get_geolocation(callback: *mut cef_get_geolocation_callback_t)
     -> ::libc::c_int;
    pub fn cef_add_cross_origin_whitelist_entry(source_origin:
                                                    *const cef_string_t,
                                                target_protocol:
                                                    *const cef_string_t,
                                                target_domain:
                                                    *const cef_string_t,
                                                allow_target_subdomains:
                                                    ::libc::c_int)
     -> ::libc::c_int;
    pub fn cef_remove_cross_origin_whitelist_entry(source_origin:
                                                       *const cef_string_t,
                                                   target_protocol:
                                                       *const cef_string_t,
                                                   target_domain:
                                                       *const cef_string_t,
                                                   allow_target_subdomains:
                                                       ::libc::c_int)
     -> ::libc::c_int;
    pub fn cef_clear_cross_origin_whitelist() -> ::libc::c_int;
    pub fn cef_get_path(key: cef_path_key_t, path: *mut cef_string_t)
     -> ::libc::c_int;
    pub fn cef_launch_process(command_line: *mut Struct__cef_command_line_t)
     -> ::libc::c_int;
    pub fn cef_begin_tracing(categories: *const cef_string_t,
                             callback: *mut Struct__cef_completion_callback_t)
     -> ::libc::c_int;
    pub fn cef_end_tracing(tracing_file: *const cef_string_t,
                           callback: *mut cef_end_tracing_callback_t)
     -> ::libc::c_int;
    pub fn cef_now_from_system_trace_time() -> int64;
    pub fn cef_parse_url(url: *const cef_string_t,
                         parts: *mut Struct__cef_urlparts_t) -> ::libc::c_int;
    pub fn cef_create_url(parts: *const Struct__cef_urlparts_t,
                          url: *mut cef_string_t) -> ::libc::c_int;
    pub fn cef_get_mime_type(extension: *const cef_string_t)
     -> cef_string_userfree_t;
    pub fn cef_get_extensions_for_mime_type(mime_type: *const cef_string_t,
                                            extensions: cef_string_list_t)
     -> ();
    pub fn cef_urlrequest_create(request: *mut Struct__cef_request_t,
                                 client: *mut Struct__cef_urlrequest_client_t)
     -> *mut cef_urlrequest_t;
    pub fn cef_xml_reader_create(stream: *mut Struct__cef_stream_reader_t,
                                 encodingType: cef_xml_encoding_type_t,
                                 URI: *const cef_string_t)
     -> *mut cef_xml_reader_t;
    pub fn cef_zip_reader_create(stream: *mut Struct__cef_stream_reader_t)
     -> *mut cef_zip_reader_t;
}

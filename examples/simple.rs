#![feature(default_type_parameter_fallback)]

extern crate cef;

use std::default::Default;

fn main() {
    let app: Option<cef::App> = None;
    let result_code = cef::execute_process(app);
    if result_code >= 0 { // The process was a helper process, so end now.
        std::process::exit(result_code as i32);
    }

    let settings = cef::Settings {
        log_file: Some("log.log"),
        locale: Some("en_GB"), // This improves CEF's grammar ;-)
        .. Default::default()
    };
    if !cef::initialize(&settings, None) {
        panic!("Initialising CEF failed. Please check the log file.")
    }

    // Set the window title and dimensions
    let window_info = cef::WindowInfo {
        window_name: Some("Hello CEF"),
        width: 1024,
        height: 768,
        .. Default::default()
    };
    // Any valid URL will do.
    cef::BrowserHost::create_browser_sync(
        &window_info, (), "http://www.cnn.com",
        &cef::BrowserSettings::new(), None);

    cef::run_message_loop();

    // This is important for cleanup and stopping helper processes
    cef::shutdown();
}

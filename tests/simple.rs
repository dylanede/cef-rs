extern crate cef;

use cef::Settings;
use cef::WindowInfo;
use cef::string::CefString;

#[test]
fn initialisation() {
    if cef::execute_process::<()>(None) >= 0 {
        return;
    }

    let mut settings = Settings::new();
    settings.log_file = CefString::from_str("log.log");
    settings.locale = CefString::from_str("en_GB");
    println!("initialising");
    if !cef::initialize::<()>(&settings, None) {
        panic!("Initialising CEF failed. Please check the logfile.")
    }
    let mut window_info = WindowInfo::new();
    window_info.set_windowless_rendering(true);
    window_info.set_transparent_painting(true);

    cef::shutdown();
}

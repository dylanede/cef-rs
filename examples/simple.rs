extern crate cef;

use cef::Settings;
use cef::BrowserHost;
use cef::WindowInfo;
use cef::BrowserClientWrapper;
use cef::BrowserSettings;
use cef::CefRc;
use cef::Browser;
use cef::string::CefString;
use cef::Void;

fn main() {
    let result_code = cef::execute_process::<Void>(None);
    if result_code >= 0 {
        std::os::set_exit_status(result_code as isize);
        return;
    }

    let mut settings = Settings::new();
    settings.log_file = CefString::from_str("log.log");
    settings.locale = CefString::from_str("en_GB");
    println!("initialising");
    if !cef::initialize::<Void>(&settings, None) {
        panic!("Initialising CEF failed. Please check the logfile.")
    }
    let mut window_info = WindowInfo::new();
    //window_info.windowless_rendering_enabled.set(true);
    //window_info.transparent_painting_enabled.set(true);
    let settings = BrowserSettings::new();
    let url = CefString::from_str("http://www.google.com");
    struct MyClient;
    let browser = BrowserHost::create_browser_sync(
        &window_info,
        BrowserClientWrapper::new(()),
        &url,
        &settings,
        None);

    cef::run_message_loop();

    cef::shutdown();
}

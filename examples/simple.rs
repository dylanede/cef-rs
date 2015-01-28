extern crate cef;

use cef::Settings;
use cef::BrowserHost;
use cef::WindowInfo;
use cef::BrowserClientWrapper;
use cef::BrowserSettings;
use cef::CefRc;
use cef::Browser;
use cef::string::CefString;

fn main() {
    let result_code = cef::execute_process::<()>(None);
    if result_code >= 0 {
        std::os::set_exit_status(result_code as isize);
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
    //window_info.set_windowless_rendering(true);
    //window_info.set_transparent_painting(true);
    let settings = BrowserSettings::new();
    let url = CefString::from_str("http://www.google.com");
    let browser = BrowserHost::create_browser_sync(&window_info, BrowserClientWrapper::new(()), &url, &settings, None);

    cef::run_message_loop();

    cef::shutdown();
}

extern crate cef;

use std::default::Default;

fn main() {
    /*
    cef::EnableHighDpiSupport();
    */

    let app: Option<()> = Some(()); // TODO: Investigate this [1]unit business.
    let result_code = cef::execute_process(app);
    if result_code != -1 {
        // The process was a helper process, so end now.
        std::process::exit(result_code as i32);
    }

    let settings = cef::Settings {
        log_file: Some("hello_cef.log"),
        log_severity: cef::LogSeverity::LOGSEVERITY_DEFAULT,
        locale: Some("en_GB"), // This improves CEF's grammar ;-)
        ..Default::default()
    };
    if !cef::initialize(&settings, app) {
        panic!("Initialising CEF failed. Please check the log file.")
    }

    // Set the window title and dimensions
    let window_info = cef::WindowInfo {
        window_name: Some("Hello CEF"),
        width: 1024,
        height: 768,
        ..Default::default()
    };

    // Any valid URL will do.
    println!("create_browser_sync");    
    cef::BrowserHost::create_browser_sync(
        &window_info,
        (), // TODO: Investigate this [1]unit busines.
        "http://www.google.com",
        &cef::BrowserSettings::new(), /*,
                                          None*/
    );

    // TODO: Closing the window does not exit the message loop
    println!("run_message_loop");
    cef::run_message_loop();

    // This is important for cleanup and stopping helper processes
    println!("shutdown");
    cef::shutdown();

    // [1] The unit business.
    //     Look into replacing the unit usage with somthing less weird.
    //     Builder pattern or DefaultBrowserClient::new() ?
    //                        DefaultCefApp::new()        ?
    //     
}

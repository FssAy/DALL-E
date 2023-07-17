#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod js;
mod injector;


fn main() {
    tauri::Builder::default()
        .on_page_load(|window, payload| {
            let url = payload.url();

            if url.starts_with("https://pixlr.com") && url.ends_with("x/#generator") {
                window.eval(js::BLOCKER).unwrap();  // should never fail
            } else if url.starts_with("https://tauri.localhost") {

            } else {
                // "Crash" on any unwanted redirect
                panic!();
            }
        })
        .invoke_handler(tauri::generate_handler![injector::inject])
        .run(tauri::generate_context!())
        .expect("error while running the application");
}

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use rdev::{listen, EventType, set_is_main_thread};

fn main() {
    tauri::Builder::default()
    .setup(|app| {
        let app_handle = app.handle();

        // NOTE: needed on macOS
        set_is_main_thread(false);

        // Spawn the input listener on a different thread
        std::thread::spawn(move || {
            if let Err(error) = listen(move |event| {
                match event.event_type {
                    EventType::KeyPress(_key) => {
                        app_handle.emit_all("key-pressed", ()).map_err(|e| {
                            println!("Error: {}", e.to_string())
                        }).unwrap();

                        // DEBUG PRINT
                        // println!("Key: {:?}", _key);
                    },
                    _ => {}
                }
            }) {
                println!("Error: {:?}", error)
            }

        });


        Ok(())
    })
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
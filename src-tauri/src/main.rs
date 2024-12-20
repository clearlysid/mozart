// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use rdev::{listen, Event, EventType};

fn callback(event: Event) {
    match event.event_type {
        EventType::KeyPress(key) => {
            println!("Key: {:?}", key);
        },
        _ => {}
    }
}

fn main() {
    tauri::Builder::default()
    .setup(|_| {
        if let Err(error) = listen(callback) {
            println!("Error: {:?}", error)
        }
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
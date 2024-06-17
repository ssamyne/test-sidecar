// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::thread::sleep;

use tauri::{ Manager };

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// define the payload struct
#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

// make the command
#[tauri::command]
async fn test_app_handle(app: tauri::AppHandle) {
    app.emit_all("event-name", Payload { message: "Tauri is awesome!".into() }).unwrap();
}

fn main() {
    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![greet])
        .setup(|app| {
            // Get the AppHandle to use in the async block
            let app_handle = app.handle();

            // Use async runtime to call async function in setup
            tauri::async_runtime::spawn(async move {
                // Call the test_app_handle function with the AppHandle
                test_app_handle(app_handle).await;
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("failed to run app");
}

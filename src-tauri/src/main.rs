// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Window;
use std::sync::{ Arc, Mutex };
use std::thread::{ self, ThreadId };
use std::time::Duration;

struct ThreadIdState {
    thread_id: ThreadId,
    is_running: bool,
}

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

#[tauri::command]
async fn init_process(
    window: Window,
    serial_port: String,
    baud_rate: String,
    byte_size: String,
    parity: String,
    stop_bits: String,
    timeout: String,
    stop_flag: tauri::State<'_, Arc<Mutex<bool>>>,
    thread_id_state: tauri::State<'_, Arc<Mutex<ThreadIdState>>>
) -> Result<(), String> {
    // Use Arc and Mutex for the counter and stop flag.
    let counter = Arc::new(Mutex::new(0));
    let window = window.clone();
    let stop_flag_clone = stop_flag.inner().clone();
    let mut thread_id_state = thread_id_state.inner().lock().unwrap();

    if thread_id_state.is_running {
        println!(
            "Process is already running on {:?} {:?}",
            thread_id_state.thread_id,
            thread_id_state.is_running
        );
        return Err("Process is already running".to_string());
    }

    thread_id_state.is_running = true;
    let thread_id_state_copy = ThreadIdState {
        thread_id: thread_id_state.thread_id,
        is_running: thread_id_state.is_running,
    };
    drop(thread_id_state);

    let join_handle = thread::spawn(move || {
        let spawned_thread_id: ThreadId = thread::current().id();
        let mut thread_id_state = thread_id_state_copy;
        thread_id_state.thread_id = thread::current().id();

        println!("Spawned thread ID: {:?}", spawned_thread_id);

        loop {
            let mut num = counter.lock().unwrap();
            // Check if the stop flag is set
            if *stop_flag_clone.lock().unwrap() {
                println!("Stop Thread ID: {:?}", spawned_thread_id);

                *num = 0;
                window.emit("event-name", Payload { message: num.to_string() }).unwrap();

                *stop_flag_clone.lock().unwrap() = false;
                break; // Exit the loop
            }

            println!(
                "running in serial_port = {}, baud_rate = {}, byte_size = {}, parity = {}, stop_bits = {}, timeout: = {}",
                serial_port,
                baud_rate,
                byte_size,
                parity,
                stop_bits,
                timeout
            );

            window.emit("event-name", Payload { message: num.to_string() }).unwrap();
            *num += 1;
            thread::sleep(Duration::from_millis(1000)); // Adjust the sleep duration as needed.
        }
    });

    // Wait for the thread to finish, or stop it if needed
    join_handle.join().map_err(|_| "Thread panicked".to_string())?;
    Ok(())
}

#[tauri::command]
async fn stop_process(
    stop_flag: tauri::State<'_, Arc<Mutex<bool>>>,
    thread_id_state: tauri::State<'_, Arc<Mutex<ThreadIdState>>>
) -> Result<(), String> {
    let stop_flag = stop_flag.inner();
    let mut thread_id_state = thread_id_state.inner().lock().unwrap();
    thread_id_state.is_running = false;
    *stop_flag.lock().unwrap() = true;

    Ok(())
}

fn main() {
    let stop_flag = Arc::new(Mutex::new(false));
    let thread_id_state = Arc::new(
        Mutex::new(ThreadIdState {
            thread_id: thread::current().id(), // Convert NonZeroU64 to u64
            is_running: false,
        })
    );

    tauri::Builder
        ::default()
        .invoke_handler(tauri::generate_handler![init_process, greet, stop_process])
        .manage(stop_flag.clone())
        .manage(thread_id_state.clone())
        .run(tauri::generate_context!())
        .expect("failed to run app");
}

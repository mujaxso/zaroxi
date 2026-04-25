// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::panic;

fn main() {
    // Set a custom panic hook to capture crash location
    panic::set_hook(Box::new(|info| {
        eprintln!("PANIC: {}", info);
        if let Some(location) = info.location() {
            eprintln!("  at {}:{}", location.file(), location.line());
        }
        if let Some(msg) = info.payload().downcast_ref::<&str>() {
            eprintln!("  message: {}", msg);
        } else if let Some(msg) = info.payload().downcast_ref::<String>() {
            eprintln!("  message: {}", msg);
        }
        // Print backtrace if available
        let backtrace = std::backtrace::Backtrace::new();
        eprintln!("{:?}", backtrace);
        // Abort to prevent further damage
        std::process::abort();
    }));

    if let Err(e) = desktop::run() {
        eprintln!("Error while running tauri application: {}", e);
        std::process::exit(1);
    }
}

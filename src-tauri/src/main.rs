#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod commands;
pub mod db;
pub mod errors;
pub mod models;
pub mod schema;

use crate::commands::*;

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            db_commands::get_apartments,
            db_commands::get_guests
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

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
            db_commands::get_guests_in_apartment,
            db_commands::get_apartment_by_id
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

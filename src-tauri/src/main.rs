#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

pub mod commands;
pub mod db;
pub mod errors;
pub mod models;
pub mod schema;

use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

use crate::commands::*;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("./migrations");

fn main() {
    let mut connection = db::establish_connection().expect("errore connessione db");
    connection
        .run_pending_migrations(MIGRATIONS)
        .expect("Error on migrating");

    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            db_commands::get_apartments,
            db_commands::get_guests_in_apartment,
            db_commands::get_apartment_by_id,
            db_commands::open_apartment
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

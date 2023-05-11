use diesel::prelude::*;
use diesel::SqliteConnection;
use std::fs;
use std::path::Path;
use tauri::api::path::local_data_dir;

use crate::errors::MyError;

pub fn establish_connection() -> Result<SqliteConnection, MyError> {
    let identifier = "com.mitch.casine";
    let db_name = "casine.db";

    let local_data_dir = local_data_dir().ok_or(MyError::LocalDataDirNotFound)?;
    let data_path = Path::new(&local_data_dir).join(identifier);
    fs::create_dir_all(&data_path).or(Err(MyError::DataDirCreationFailed))?;

    let db_path = data_path.join(db_name);

    return SqliteConnection::establish(db_path.to_str().unwrap())
        .map_err(MyError::DatabaseConnectionError);
}

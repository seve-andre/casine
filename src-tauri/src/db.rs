use crate::errors::MyError;
use diesel::prelude::*;
use diesel::SqliteConnection;
use dotenvy::dotenv;
use std::env;

pub fn establish_connection() -> Result<SqliteConnection, MyError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    SqliteConnection::establish(&database_url).map_err(MyError::DatabaseConnectionError)
    // .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

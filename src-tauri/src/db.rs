use diesel::prelude::*;
use diesel::MysqlConnection;
use dotenvy::dotenv;
use std::env;

use crate::errors::MyError;

pub fn establish_connection() -> Result<MysqlConnection, MyError> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    MysqlConnection::establish(&database_url).map_err(MyError::DatabaseConnectionError)
}

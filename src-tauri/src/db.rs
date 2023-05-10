use diesel::prelude::*;
use diesel::MysqlConnection;

use crate::errors::MyError;

pub fn establish_connection() -> Result<MysqlConnection, MyError> {
    MysqlConnection::establish("mysql://mitch:iHtLfC75&qM2Qj@localhost/casine")
        .map_err(MyError::DatabaseConnectionError)
}

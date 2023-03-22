// see https://tauri.app/v1/guides/features/command
use crate::{db::establish_connection, errors::MyError, models::apartment::Apartment, schema};
use diesel::prelude::*;

/*
    SELECT *
    FROM apartments
*/
#[tauri::command]
pub async fn get_apartments() -> Result<Vec<Apartment>, MyError> {
    use schema::apartments::dsl::*;

    let connection = &mut establish_connection()?;

    return apartments
        .load::<Apartment>(connection)
        .map_err(MyError::DatabaseQueryError);
}

// see at https://docs.diesel.rs/2.0.x/diesel/associations/index.html
// #[tauri::command]
// pub async fn get_apartments_by_house() -> Result<Vec<(House, Vec<Apartment>)>, MyError>

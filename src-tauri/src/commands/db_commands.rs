// see https://tauri.app/v1/guides/features/command
use crate::{
    db::establish_connection,
    errors::MyError,
    models::{apartment::Apartment, house::House},
    schema,
};
use diesel::prelude::*;

/*
    SELECT *
    FROM apartments
    WHERE house_id = (
        SELECT id
        FROM houses
        WHERE house_name = ?
    )
*/
#[tauri::command]
pub async fn get_apartments_in_house(p_house_name: String) -> Result<Vec<Apartment>, MyError> {
    use schema::houses::dsl::*;

    let connection = &mut establish_connection()?;

    let house = houses
        .filter(house_name.eq(p_house_name))
        .get_result::<House>(connection)
        .map_err(MyError::DatabaseQueryError)?;

    return Apartment::belonging_to(&house)
        .load::<Apartment>(connection)
        .map_err(MyError::DatabaseQueryError);
}

// see at https://docs.diesel.rs/2.0.x/diesel/associations/index.html
// #[tauri::command]
// pub async fn get_apartments_by_house() -> Result<Vec<(House, Vec<Apartment>)>, MyError>

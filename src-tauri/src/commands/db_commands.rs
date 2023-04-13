// see https://tauri.app/v1/guides/features/command
use crate::{
    db::establish_connection,
    errors::MyError,
    models::{apartment::Apartment, guest::Guest},
    schema::{self, apartments::apartment_number, rents},
};
use chrono::{NaiveDate, NaiveTime, Utc};
use diesel::{prelude::*, sql_types::Bool};

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

pub async fn get_apartment_by_house_name_and_number(
    p_house_name: String,
    p_apartment_number: i32,
) -> Result<i32, MyError> {
    use schema::apartments::dsl::*;

    let connection = &mut establish_connection()?;

    return apartments
        .filter(apartment_number.eq(p_apartment_number))
        .filter(house_name.eq(p_house_name))
        .select(id)
        .get_result::<i32>(connection)
        .map_err(MyError::DatabaseQueryError);
}

/*
    SELECT *
    FROM guests
*/
pub async fn get_guests(
    p_house_name: String,
    p_apartment_number: i32,
) -> Result<Vec<Guest>, MyError> {
    use schema::rents::dsl::*;

    let connection = &mut establish_connection()?;

    // get apartment_id from house name and number
    let apartment_id_result =
        get_apartment_by_house_name_and_number(p_house_name, p_apartment_number).await?;

    // based on date and apartment_id, get group_id
    let today_date = Utc::now().date_naive();

    let group_id_result = rents
        .filter(apartment_id.eq(apartment_id_result))
        .filter(start_date.le(&today_date))
        .filter(end_date.ge(&today_date))
        .select(group_id)
        .get_result::<i32>(connection);

    // from group_id, get guest_id -> Guest

    return Result::Ok(vec![]);
}

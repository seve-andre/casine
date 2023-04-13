// see https://tauri.app/v1/guides/features/command
use crate::{
    db::establish_connection,
    errors::MyError,
    models::{apartment::Apartment, guest::Guest, rent::Rent},
    schema,
};
use chrono::Utc;
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

fn get_apartment_by_house_name_and_number(
    p_house_name: String,
    p_apartment_number: i32,
) -> Result<Apartment, MyError> {
    use schema::apartments::dsl::*;

    let connection = &mut establish_connection()?;

    return apartments
        .filter(apartment_number.eq(p_apartment_number))
        .filter(house_name.eq(p_house_name))
        .get_result::<Apartment>(connection)
        .map_err(MyError::DatabaseQueryError);
}

fn get_group_id_by_apartment(apartment: &Apartment) -> Result<i32, MyError> {
    use schema::rents::dsl::*;

    let connection = &mut establish_connection()?;
    let today_date = Utc::now().date_naive();

    println!("fuck you");

    return Rent::belonging_to(apartment)
        // .filter(start_date.le(&today_date))
        // .filter(end_date.ge(&today_date))
        .select(group_id)
        .get_result::<i32>(connection)
        .map_err(MyError::DatabaseQueryError);
}

fn get_guests_ids_by_group_id(group_id_result: i32) -> Result<Vec<i32>, MyError> {
    use schema::group_members::dsl::*;

    let connection = &mut establish_connection()?;

    return group_members
        .filter(group_id.eq(&group_id_result))
        .select(guest_id)
        .load::<i32>(connection)
        .map_err(MyError::DatabaseQueryError);
}

/*
    SELECT *
    FROM guests
*/
#[tauri::command]
pub async fn get_guests(
    p_house_name: String,
    p_apartment_number: i32,
) -> Result<Vec<Guest>, MyError> {
    use schema::guests::dsl::*;

    let connection = &mut establish_connection()?;

    // get apartment_id from house name and number
    let apartment = get_apartment_by_house_name_and_number(p_house_name, p_apartment_number)?;

    // based on date and apartment_id, get group_id

    let group_id_result = get_group_id_by_apartment(&apartment)?;
    println!("group_id_result: {:?}", group_id_result);

    // from group_id, get guest_id
    let guest_ids = get_guests_ids_by_group_id(group_id_result)?;

    // guest_ids -> Vec<Guest>
    return guests
        .filter(id.eq_any(&guest_ids))
        .load::<Guest>(connection)
        .map_err(MyError::DatabaseQueryError);
}

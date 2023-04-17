// see https://tauri.app/v1/guides/features/command
use crate::{
    db::establish_connection,
    errors::MyError,
    models::{apartment::Apartment, guest::Guest, rent::Rent},
    schema::{self, rents},
};
use chrono::{Local, NaiveDate};
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

type DB = diesel::mysql::Mysql;
fn is_date_in_range(
    date: NaiveDate,
) -> Box<dyn BoxableExpression<rents::table, DB, SqlType = Bool>> {
    return Box::new(rents::start_date.le(date).and(rents::end_date.ge(date)));
}

fn get_group_id_by_apartment(apartment: &Apartment) -> Result<i32, MyError> {
    use schema::rents::dsl::*;

    let connection = &mut establish_connection()?;
    let today_date = Local::now().date_naive();

    // problem is, it
    return Rent::belonging_to(apartment)
        .filter(is_date_in_range(today_date))
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
    TODO
*/
#[tauri::command]
pub async fn get_guests(house_name: String, apartment_number: i32) -> Result<Vec<Guest>, MyError> {
    use schema::guests::dsl::*;

    let connection = &mut establish_connection()?;

    let apartment = get_apartment_by_house_name_and_number(house_name, apartment_number)?;

    let group_id_result = get_group_id_by_apartment(&apartment);
    let actual_group_id = match group_id_result {
        Ok(it) => it,
        Err(_) => return Result::Ok(vec![]),
    };

    let guest_ids = get_guests_ids_by_group_id(actual_group_id)?;

    return guests
        .filter(id.eq_any(&guest_ids))
        .load::<Guest>(connection)
        .map_err(MyError::DatabaseQueryError);
}

#[tauri::command]
pub async fn get_apartment_by_id(apartment_id: i32) -> Result<Apartment, MyError> {
    use schema::apartments::dsl::*;

    let connection = &mut establish_connection()?;

    return apartments
        .find(apartment_id)
        .first(connection)
        .map_err(MyError::DatabaseQueryError);
}

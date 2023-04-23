// see https://tauri.app/v1/guides/features/command
use crate::{
    db::establish_connection,
    errors::MyError,
    models::{
        apartment::Apartment,
        group::{Group, GroupMember, NewGroup},
        guest::{Guest, NewGuest},
        rent::{NewRent, Rent},
    },
    schema::{self, rents},
};
use chrono::{Local, NaiveDate};
use diesel::{insert_into, prelude::*, sql_types::Bool};

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

#[tauri::command]
pub async fn get_guests_in_apartment(apartment_id: i32) -> Result<Vec<Guest>, MyError> {
    use schema::apartments;
    use schema::guests::dsl::*;

    let connection = &mut establish_connection()?;

    let apartment = apartments::table
        .find(apartment_id)
        .get_result::<Apartment>(connection)?;

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

#[tauri::command]
pub fn open_apartment(
    apartment_id: i32,
    start_date: NaiveDate,
    end_date: NaiveDate,
    new_guest: NewGuest,
    new_group: NewGroup,
) -> Result<(), MyError> {
    use schema::rents;

    let connection = &mut establish_connection()?;

    let guest_group = create_new_group(new_group)?;
    let group_id = guest_group.id;

    add_guest_to_group(new_guest, group_id, true)?;

    let new_rent = NewRent {
        start_date: &start_date,
        end_date: &end_date,
        group_id,
        apartment_id,
    };

    insert_into(rents::table)
        .values(&new_rent)
        .execute(connection)?;

    return Ok(());
}

fn create_new_group(group: NewGroup) -> Result<Group, MyError> {
    use schema::groupz::dsl::*;

    let connection = &mut establish_connection()?;

    let inserted_group = connection.transaction::<Group, MyError, _>(|connection| {
        insert_into(groupz).values(group).execute(connection)?;

        groupz
            .order(id.desc())
            .first(connection)
            .map_err(MyError::DatabaseQueryError)
    });

    return inserted_group;
}

#[tauri::command]
pub fn add_guest_to_group(guest: NewGuest, group_id: i32, is_leader: bool) -> Result<(), MyError> {
    use schema::group_members;
    use schema::guests;

    let connection = &mut establish_connection()?;

    let inserted_guest = connection.transaction::<Guest, MyError, _>(|connection| {
        insert_into(guests::table)
            .values(guest)
            .execute(connection)?;

        guests::table
            .order(guests::id.desc())
            .first(connection)
            .map_err(MyError::DatabaseQueryError)
    })?;

    let new_group_member = GroupMember {
        guest_id: inserted_guest.id,
        group_id,
        is_leader,
    };

    insert_into(group_members::table)
        .values(&new_group_member)
        .execute(connection)?;

    return Ok(());
}

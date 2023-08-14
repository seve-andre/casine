// see https://tauri.app/v1/guides/features/command
use crate::{
    db::establish_connection,
    errors::MyError,
    models::{
        apartment::Apartment,
        group::{Group, GroupMember, NewGroup},
        guest::{Guest, NewGuest},
        house::{House, HouseWithApartments},
        rent::{NewRent, Rent},
    },
    schema::{self, rents},
};
use chrono::{Local, NaiveDate};
use diesel::{insert_into, prelude::*, sql_types::Bool};
use serde::Serialize;

/*
    SELECT *
    FROM apartments
*/
#[tauri::command]
pub async fn get_apartments() -> Result<Vec<HouseWithApartments>, MyError> {
    use schema::houses;

    let connection = &mut establish_connection()?;

    let all_houses = houses::table
        .select(House::as_select())
        .load::<House>(connection)?;

    let apartments = Apartment::belonging_to(&all_houses)
        .select(Apartment::as_select())
        .load(connection)?;

    Ok(apartments
        .grouped_by(&all_houses)
        .into_iter()
        .zip(all_houses)
        .map(|(apartments, house)| HouseWithApartments { house, apartments })
        .collect::<Vec<HouseWithApartments>>())
}

type DB = diesel::sqlite::Sqlite;
fn is_date_in_range(
    date: NaiveDate,
) -> Box<dyn BoxableExpression<rents::table, DB, SqlType = Bool>> {
    Box::new(rents::start_date.le(date).and(rents::end_date.ge(date)))
}

fn get_guests_ids_by_group_id(group_id: i32) -> Result<Vec<i32>, MyError> {
    use schema::group_members;

    let connection = &mut establish_connection()?;

    group_members::table
        .filter(group_members::group_id.eq(group_id))
        .select(group_members::guest_id)
        .load::<i32>(connection)
        .map_err(MyError::DatabaseQueryError)
}

#[tauri::command]
pub async fn get_guests_in_apartment(apartment_id: i32) -> Result<Option<Vec<Guest>>, MyError> {
    use schema::guests;

    let connection = &mut establish_connection()?;

    let maybe_group = get_group_in_apartment(apartment_id).await?;

    let group = match maybe_group {
        Some(group) => group,
        None => return Ok(None),
    };

    let guest_ids = get_guests_ids_by_group_id(group.id)?;

    guests::table
        .filter(guests::id.eq_any(&guest_ids))
        .load::<Guest>(connection)
        .optional()
        .map_err(MyError::DatabaseQueryError)
}

#[tauri::command]
pub async fn get_apartment_by_id(apartment_id: i32) -> Result<Apartment, MyError> {
    use schema::apartments::dsl::*;

    let connection = &mut establish_connection()?;

    apartments
        .find(apartment_id)
        .first(connection)
        .map_err(MyError::DatabaseQueryError)
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
        start_date,
        end_date,
        group_id,
        apartment_id,
    };

    insert_into(rents::table)
        .values(&new_rent)
        .execute(connection)?;

    Ok(())
}

fn create_new_group(group: NewGroup) -> Result<Group, MyError> {
    use schema::groupz::dsl::*;

    let connection = &mut establish_connection()?;

    connection.transaction::<Group, MyError, _>(|connection| {
        insert_into(groupz).values(group).execute(connection)?;

        groupz
            .order(id.desc())
            .first(connection)
            .map_err(MyError::DatabaseQueryError)
    })
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

    Ok(())
}

#[tauri::command]
pub async fn get_group_in_apartment(apartment_id: i32) -> Result<Option<Group>, MyError> {
    use schema::groupz;

    let connection = &mut establish_connection()?;
    let maybe_rent = get_rent_in_apartment(apartment_id).await?;

    let rent = match maybe_rent {
        Some(rent) => rent,
        None => return Ok(None),
    };

    groupz::table
        .filter(groupz::id.eq(rent.group_id))
        .first(connection)
        .optional()
        .map_err(MyError::DatabaseQueryError)
}

#[tauri::command]
pub async fn get_rent_in_apartment(apartment_id: i32) -> Result<Option<Rent>, MyError> {
    use schema::rents;

    let connection = &mut establish_connection()?;
    let today_date = Local::now().date_naive();

    return rents::table
        .filter(rents::apartment_id.eq(apartment_id))
        .filter(is_date_in_range(today_date))
        .select(Rent::as_select())
        .first(connection)
        .optional()
        .map_err(MyError::DatabaseQueryError);
}

#[derive(Serialize, Debug)]
pub struct RentalDetails {
    apartment: Apartment,
    rent: Option<Rent>,
    guests: Option<Vec<Guest>>,
    group: Option<Group>,
}

#[tauri::command]
pub async fn get_rental_details(apartment_id: i32) -> Result<RentalDetails, MyError> {
    let (apartment_result, rent_result, guests_result, group_result) = tokio::join!(
        get_apartment_by_id(apartment_id),
        get_rent_in_apartment(apartment_id),
        get_guests_in_apartment(apartment_id),
        get_group_in_apartment(apartment_id),
    );

    let rental_details = RentalDetails {
        apartment: apartment_result?,
        rent: rent_result?,
        guests: guests_result?,
        group: group_result?,
    };

    Ok(rental_details)
}

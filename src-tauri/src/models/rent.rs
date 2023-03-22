use crate::schema::rents;
use chrono::NaiveDate;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize)]
#[diesel(table_name = rents)]
pub struct NewRent<'a> {
    start_date: &'a NaiveDate,
    end_date: &'a NaiveDate,
    group_id: i32,
    apartment_id: i32,
}

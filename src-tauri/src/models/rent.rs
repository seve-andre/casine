use crate::schema::rents;
use chrono::NaiveDate;
use diesel::Insertable;
use serde::Serialize;

#[derive(Insertable, Serialize)]
#[diesel(table_name = rents)]
pub struct NewRent<'a> {
    pub start_date: &'a NaiveDate,
    pub end_date: &'a NaiveDate,
    pub group_id: i32,
    pub apartment_id: i32,
}

use crate::schema::rents;
use chrono::NaiveDate;
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

use super::apartment::Apartment;
use super::group::Group;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[diesel(table_name = rents, belongs_to(Group), belongs_to(Apartment))]
pub struct Rent {
    pub id: i32,
    pub apartment_id: i32,
    pub group_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = rents)]
pub struct NewRent<'a> {
    pub start_date: &'a NaiveDate,
    pub end_date: &'a NaiveDate,
    pub group_id: i32,
    pub apartment_id: i32,
}

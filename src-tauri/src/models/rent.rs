use crate::schema::rents;
use chrono::NaiveDate;
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use super::apartment::Apartment;
use super::group::Group;

#[derive(Identifiable, Selectable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Group))]
#[diesel(belongs_to(Apartment))]
#[diesel(table_name = rents)]
pub struct Rent {
    pub id: i32,
    pub apartment_id: i32,
    pub group_id: i32,
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
}

#[derive(Insertable, Deserialize)]
#[diesel(table_name = rents)]
pub struct NewRent {
    pub start_date: NaiveDate,
    pub end_date: NaiveDate,
    pub group_id: i32,
    pub apartment_id: i32,
}

use crate::schema::guests;
use chrono::NaiveDate;
use diesel::{Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Selectable, Queryable, Serialize, Deserialize, Debug)]
#[diesel(table_name = guests)]
pub struct Guest {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = guests)]
pub struct NewGuest<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub birth_date: &'a NaiveDate,
}

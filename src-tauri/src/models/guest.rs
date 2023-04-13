use crate::schema::guests;
use chrono::NaiveDate;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = guests)]
pub struct Guest {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub birth_date: NaiveDate,
    pub phone_number: Option<String>,
    pub nationality: Option<String>,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = guests)]
pub struct NewGuest<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub birth_date: &'a NaiveDate,
}

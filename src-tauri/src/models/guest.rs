use crate::schema::guests;
use chrono::NaiveDate;
use diesel::{Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = guests)]
pub struct Guest {
    id: i32,
    first_name: String,
    last_name: String,
    birth_date: NaiveDate,
    /*pub birthplace: Option<String>,
    pub phone_number: Option<String>,
    pub nationality: Option<String>,
    pub residence: Option<String>,*/
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = guests)]
pub struct NewGuest<'a> {
    first_name: &'a str,
    last_name: &'a str,
    birth_date: &'a NaiveDate,
}

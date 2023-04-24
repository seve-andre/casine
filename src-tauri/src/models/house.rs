use crate::schema::houses;
use diesel::{Identifiable, Queryable};
use serde::Serialize;

#[derive(Identifiable, Queryable, Serialize)]
#[diesel(table_name = houses)]
#[diesel(primary_key(house_name))]
pub struct House {
    pub house_name: char,
    pub street_type: String,
    pub street_name: String,
    pub street_number: i32,
}

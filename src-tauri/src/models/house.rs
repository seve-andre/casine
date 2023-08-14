use crate::schema::houses;
use diesel::{Identifiable, Queryable, Selectable};
use serde::Serialize;

use super::apartment::Apartment;

#[derive(Identifiable, Queryable, Selectable, Serialize, Debug, PartialEq, Eq)]
#[diesel(table_name = houses)]
#[diesel(primary_key(house_name))]
pub struct House {
    pub house_name: String,
    pub street_type: String,
    pub street_name: String,
    pub street_number: i32,
}

#[derive(Serialize)]
pub struct HouseWithApartments {
    pub house: House,
    pub apartments: Vec<Apartment>,
}

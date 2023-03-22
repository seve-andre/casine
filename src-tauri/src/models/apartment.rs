use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::house::House;
use crate::schema::apartments;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(House, foreign_key=house_name), table_name = apartments)]
pub struct Apartment {
    id: i32,
    house_name: String,
    apartment_number: i32,
}

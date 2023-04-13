use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

use super::house::House;
use crate::schema::apartments;

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[diesel(table_name = apartments, belongs_to(House, foreign_key=house_name))]
pub struct Apartment {
    pub id: i32,
    pub house_name: String,
    pub apartment_number: i32,
}

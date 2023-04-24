use diesel::{Associations, Identifiable, Queryable, Selectable};
use serde::Serialize;

use super::house::House;
use crate::schema::apartments;

#[derive(Identifiable, Selectable, Queryable, Associations, Serialize, Debug)]
#[diesel(belongs_to(House, foreign_key=house_name))]
#[diesel(table_name = apartments)]
pub struct Apartment {
    pub id: i32,
    pub house_name: String,
    pub apartment_number: i32,
}

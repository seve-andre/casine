use super::house::House;
use crate::schema::apartments;
use diesel::{Associations, Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(House), table_name = apartments)]
pub struct Apartment {
    pub id: i32,
    pub house_id: i32,
    pub apartment_number: i32,
}

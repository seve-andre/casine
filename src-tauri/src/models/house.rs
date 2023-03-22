use crate::schema::houses;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = houses)]
pub struct House {
    id: i32,
    house_name: String,
    street_type: String,
    street_name: String,
    street_number: i32,
}

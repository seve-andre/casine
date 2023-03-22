use crate::schema::houses;
use diesel::{Identifiable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Identifiable, Queryable, Serialize, Deserialize)]
#[diesel(table_name = houses)]
pub struct House {
    pub id: i32,
    pub house_name: String,
    pub street_type: String,
    pub street_name: String,
    pub street_number: i32,
}

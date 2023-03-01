use super::guest::Guest;
use crate::schema::{group_members, groups};
use diesel::{Associations, Identifiable, Insertable, Queryable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Identifiable, Serialize, Deserialize)]
#[diesel(table_name = groups)]
pub struct Group {
    pub id: i32,
    pub leader_id: i32,
    pub nickname: Option<String>,
}

#[derive(Insertable, Serialize)]
#[diesel(table_name = groups)]
pub struct NewGroup<'a> {
    pub nickname: Option<&'a str>,
}

#[derive(Insertable, Queryable, Associations, Serialize, Deserialize)]
#[diesel(belongs_to(Guest), belongs_to(Group))]
#[diesel(table_name = group_members)]
pub struct GroupMember {
    guest_id: i32,
    group_id: i32,
}

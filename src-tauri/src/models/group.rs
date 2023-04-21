use super::guest::Guest;
use crate::schema::{group_members, groupz};
use diesel::{Associations, Identifiable, Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Selectable, Identifiable, Serialize, Deserialize, PartialEq, Debug)]
#[diesel(table_name = groupz)]
pub struct Group {
    pub id: i32,
    pub nickname: String,
}

#[derive(Insertable, Serialize, Deserialize, Debug)]
#[diesel(table_name = groupz)]
pub struct NewGroup<'a> {
    pub nickname: &'a str,
}

#[derive(Insertable, Selectable, Queryable, Associations, Serialize, Deserialize, Debug)]
#[diesel(belongs_to(Guest))]
#[diesel(belongs_to(Group))]
#[diesel(table_name = group_members)]
#[diesel(primary_key(guest_id, group_id))]
pub struct GroupMember {
    pub guest_id: i32,
    pub group_id: i32,
    pub is_leader: bool,
}

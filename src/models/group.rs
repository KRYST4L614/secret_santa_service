use diesel::prelude::*;
pub enum GroupStatus {
  Open,
  Closed,
}

#[derive(Queryable)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub status: GroupStatus,
}
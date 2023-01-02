use super::schema::*;
use diesel::prelude::*;

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::GroupStatus"]
pub enum GroupStatus {
    Open,
    Closed,
}

#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser {
    pub name: String,
}

impl NewUser {
    pub fn new(name: String) -> NewUser {
        NewUser { name }
    }
}

#[derive(Queryable)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub status: GroupStatus,
}

#[derive(Queryable)]
pub struct GroupUser {
    pub group_id: i32,
    pub user_id: i32,
    pub role: UserRole,
    pub ward_id: i32,
}

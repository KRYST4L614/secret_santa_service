use diesel::prelude::*;

#[derive(Debug, diesel_derive_enum::DbEnum)]
#[DieselTypePath = "crate::schema::sql_types::UserRole"]
pub enum UserRole {
    User,
    Admin,
}

#[derive(Queryable)]
pub struct GroupUser {
    pub group_id: i32,
    pub user_id: i32,
    pub role: UserRole,
    pub ward_id: i32,
}

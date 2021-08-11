#![allow(proc_macro_derive_resolution_fallback)]

use diesel;
use diesel::prelude::*;

use crate::model::Users;
use crate::model::NewUser;

use crate::schema::users_table;
use crate::schema::users_table::dsl::*;

pub fn create_user(new_user: NewUser, conn: &MysqlConnection) -> QueryResult<Users> {
    diesel::insert_into(users_table::table)
        .values(&new_user)
        .get_result(conn)
}

pub fn show_users(connection: &MysqlConnection) -> QueryResult<Vec<Users>> {
    users_table.limit(5)
        .load::<Users>(&*connection)
}

pub fn get_users(user_id: i32, connection: &MysqlConnection) -> QueryResult<Users> {
    users_table::table.find(user_id).get_result::<Users>(connection)
}

pub fn update_user(user_id: i32, user: Users, connection: &MysqlConnection) -> QueryResult<Users> {
    diesel::update(users_table::table.find(user_id))
        .set(&user)
        .get_result(connection)
}

pub fn delete_user(user_id: i32, connection: &MysqlConnection) -> QueryResult<usize> {
    diesel::delete(users_table::table.find(user_id))
        .execute(connection)
}


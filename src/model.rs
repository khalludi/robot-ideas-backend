#![allow(proc_macro_derive_resolution_fallback)]

use crate::schema::ideas;
use crate::schema::users_table;

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "ideas"]
pub struct Ideas {
    pub id: i32,
    pub userId: i32,
    pub body: String,
    pub likes: i32,
    pub dislikes: i32
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="ideas"]
pub struct NewIdea {
    pub userId: i32,
    pub body: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug)]
#[table_name = "users_table"]
pub struct Users {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable, Serialize, Deserialize)]
pub struct User_Model{
    pub id: i32,
    pub username: String,
}

#[derive(Insertable, Serialize, Deserialize)]
#[table_name="users_table"]
pub struct NewUser {
    pub username: String,
}

use std::env;

use diesel::result::Error;
use rocket::http::Status;
use rocket::response::status;
use rocket::serde::json::Json;

use crate::connection::DbConn;
use crate::model::Users;
use crate::model::NewUser;
use crate::repository;

#[get("/")]
pub fn all_users(connection: DbConn) -> Json<Users> {
    Json(Users{ id: 12, username: "bobby".to_string()})

    //repository::show_users(&connection)
    //    .map(|post| Json(post))
    //    .map_err(|error| error_status(error))
}
/*
#[post("/", format="application/json", data="<new_user>")]
pub fn create_user(new_user: Json<NewUser>, connection: DbConn) -> Result<status::Created<Json<Users>>, Status> {
    println!("here 0 {}", &new_user.username);
    repository::create_user(new_user.into_inner(), &connection)
        .map(|user| user_created(user))
        .map_err(|error| error_status(error))
}

#[get("/<id>")]
pub fn get_user(id: i32, connection: DbConn) -> Result<Json<Users>, Status> {
    repository::get_users(id, &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[put("/<id>", format="application/json", data="<user>")]
pub fn update_user(id: i32, user: Json<Users>, connection: DbConn) -> Result<Json<Users>, Status> {
    repository::update_user(id, user.into_inner(), &connection)
        .map(|user| Json(user))
        .map_err(|error| error_status(error))
}

#[delete("/<id>")]
pub fn delete_post(id: i32, connection: DbConn) -> Result<status::NoContent, Status> {
    repository::delete_user(id, &connection)
        .map(|_| status::NoContent)
        .map_err(|error| error_status(error))
}

fn user_created(post: Users) -> status::Created<Json<Users>> {
      println!("here final");
      status::Created::new(
          format!("{host}:{port}/post/{id}", host = host(), port = port(), id = post.id).to_string())
          .body(Json(post))
}

fn host() -> String {
      env::var("ROCKET_ADDRESS").expect("ROCKET_ADDRESS must be set")
}

fn port() -> String {
      env::var("ROCKET_PORT").expect("ROCKET_PORT must be set")
}
*/
fn error_status(error: Error) -> Status {
    match error {
        Error::NotFound => Status::NotFound,
        _ => Status::InternalServerError
    }
}


#![feature(decl_macro, proc_macro_hygiene)]
#[macro_use] extern crate diesel;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use] extern crate rocket;
extern crate rocket_contrib;
#[macro_use] extern crate serde;

use dotenv::dotenv;

mod router;
mod schema;
mod connection;
mod repository;
mod handler;
mod model;

#[launch]
fn rocket() -> _ {
      dotenv().ok();
      router::create_routes()
}

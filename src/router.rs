use rocket::routes;

use crate::connection;
use crate::handler;

use rocket::Rocket;
use rocket::Build;

pub fn create_routes() -> Rocket<Build> {
    rocket::build()
        .manage(connection::init_pool())
        .mount("/users",
            routes![
                handler::all_users,
//                handler::create_user,
//                handler::get_user,
//                handler::update_user,
//                handler::delete_post
            ],
        )
}

use rocket::routes;

use crate::connection;
use crate::handler;

pub fn create_routes() {
    rocket::build()
        .manage(connection::init_pool())
        .mount("/users",
            routes![
                handler::all_users,
                handler::create_user,
                handler::get_user,
                handler::update_user,
                handler::delete_post
            ],
        ).launch();
}

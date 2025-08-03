#[macro_use] extern crate rocket;

mod models;
mod state;
mod handlers;

use state::create_storage;
use handlers::*;

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .manage(create_storage())
        .mount("/api", routes![
            get_users,
            get_user,
            create_user,
            update_user,
            delete_user
        ])
}

#[macro_use] extern crate rocket;

mod models;
mod state;
mod handlers;

use state::create_storage;
use handlers::*;

#[rocket::launch]
async fn rocket() -> _ {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    let storage = create_storage().await.expect("Failed to connect to MongoDB");
    
    rocket::build()
        .manage(storage)
        .mount("/api", routes![
            get_users,
            get_user,
            create_user,
            update_user,
            delete_user
        ])
}

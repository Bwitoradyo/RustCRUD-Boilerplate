#[macro_use] extern crate rocket;

mod models;
mod state;
mod handlers;
mod error;
mod config;

use state::create_storage;
use handlers::*;
use config::AppConfig;

#[rocket::launch]
async fn rocket() -> _ {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    let config = AppConfig::load().expect("Failed to load configuration");
    let storage = create_storage(&config).await.expect("Failed to connect to MongoDB");
    
    rocket::build()
        .manage(storage)
        .manage(config)
        .mount("/api", routes![
            get_users,
            get_user,
            create_user,
            update_user,
            delete_user
        ])
}

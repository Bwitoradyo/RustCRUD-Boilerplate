use mongodb::{Client, Collection, options::ClientOptions};
use std::env;
use crate::models::User;

pub type UserStorage = Client;

pub async fn create_storage() -> Result<UserStorage, mongodb::error::Error> {
    let mongodb_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let mut client_options = ClientOptions::parse(&mongodb_uri).await?;
    client_options.max_pool_size = Some(10);
    client_options.min_pool_size = Some(5);
    
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub fn get_user_collection(client: &Client) -> Collection<User> {
    let database_name = env::var("DATABASE_NAME")
        .unwrap_or_else(|_| "rocketboiler".to_string());
    
    let database = client.database(&database_name);
    database.collection::<User>("users")
}

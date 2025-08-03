use mongodb::{Client, Collection, options::ClientOptions};
use crate::models::User;
use crate::error::AppResult;
use crate::config::AppConfig;

pub type UserStorage = Client;

pub async fn create_storage(config: &AppConfig) -> AppResult<UserStorage> {
    let mut client_options = ClientOptions::parse(&config.mongodb_uri).await?;
    client_options.max_pool_size = Some(config.max_pool_size);
    client_options.min_pool_size = Some(config.min_pool_size);
    
    let client = Client::with_options(client_options)?;
    Ok(client)
}

pub fn get_user_collection(client: &Client, config: &AppConfig) -> Collection<User> {
    let database = client.database(&config.database_name);
    database.collection::<User>("users")
}

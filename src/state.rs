use mongodb::{Client, Database, Collection};
use std::env;
use crate::models::User;

pub type UserStorage = Database;

pub async fn create_storage() -> Result<UserStorage, mongodb::error::Error> {
    let mongodb_uri = env::var("MONGODB_URI")
        .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
    
    let database_name = env::var("DATABASE_NAME")
        .unwrap_or_else(|_| "rocketboiler".to_string());
    
    let client = Client::with_uri_str(&mongodb_uri).await?;
    let database = client.database(&database_name);
    Ok(database)
}

pub fn get_user_collection(db: &Database) -> Collection<User> {
    db.collection::<User>("users")
}

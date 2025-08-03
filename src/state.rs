use std::sync::{Arc, Mutex};
use crate::models::User;

pub type UserStorage = Arc<Mutex<Vec<User>>>;

pub fn create_storage() -> UserStorage {
    Arc::new(Mutex::new(Vec::new()))
}

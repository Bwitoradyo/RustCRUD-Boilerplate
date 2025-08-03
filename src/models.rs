use serde::{Deserialize, Serialize};
use bson::oid::ObjectId;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateUser {
    #[validate(length(min = 2, max = 50, message = "Name must be between 2 and 50 characters"))]
    pub name: String,
    #[validate(email(message = "Invalid email format"))]
    pub email: String,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdateUser {
    #[validate(length(min = 2, max = 50, message = "Name must be between 2 and 50 characters"))]
    pub name: Option<String>,
    #[validate(email(message = "Invalid email format"))]
    pub email: Option<String>,
}

impl User {
    pub fn new(name: String, email: String) -> Self {
        Self {
            id: None,
            name,
            email,
        }
    }

    pub fn update(&mut self, update: UpdateUser) {
        if let Some(name) = update.name {
            self.name = name;
        }
        if let Some(email) = update.email {
            self.email = email;
        }
    }
}

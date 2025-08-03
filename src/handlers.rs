use rocket::{State, serde::json::Json, response::status};
use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use crate::models::{User, CreateUser, UpdateUser};
use crate::state::{UserStorage, get_user_collection};
use crate::error::{AppError, AppResult};

#[rocket::get("/users")]
pub async fn get_users(storage: &State<UserStorage>) -> AppResult<Json<Vec<User>>> {
    let collection = get_user_collection(storage);
    
    match collection.find(doc! {}, None).await {
        Ok(mut cursor) => {
            let mut users = Vec::new();
            while let Some(user) = cursor.try_next().await? {
                users.push(user);
            }
            Ok(Json(users))
        },
        Err(err) => Err(AppError::from(err))
    }
}

#[rocket::get("/users/<id>")]
pub async fn get_user(id: String, storage: &State<UserStorage>) -> AppResult<Json<User>> {
    let object_id = ObjectId::parse_str(&id)?;
    let collection = get_user_collection(storage);
    
    match collection.find_one(doc! { "_id": object_id }, None).await? {
        Some(user) => Ok(Json(user)),
        None => Err(AppError::NotFound("User".to_string()))
    }
}

#[rocket::post("/users", data = "<create_user>")]
pub async fn create_user(create_user: Json<CreateUser>, storage: &State<UserStorage>) -> AppResult<status::Created<Json<User>>> {
    let mut user = User::new(create_user.name.clone(), create_user.email.clone());
    let collection = get_user_collection(storage);
    
    match collection.insert_one(&user, None).await? {
        result => {
            user.id = result.inserted_id.as_object_id();
            let location = format!("/users/{}", user.id.unwrap());
            Ok(status::Created::new(location).body(Json(user)))
        }
    }
}

#[rocket::put("/users/<id>", data = "<update_user>")]
pub async fn update_user(id: String, update_user: Json<UpdateUser>, storage: &State<UserStorage>) -> AppResult<Json<User>> {
    let object_id = ObjectId::parse_str(&id)?;
    let collection = get_user_collection(storage);
    
    // Find the user first
    let mut user = match collection.find_one(doc! { "_id": object_id }, None).await? {
        Some(user) => user,
        None => return Err(AppError::NotFound("User".to_string()))
    };
    
    // Update the user
    user.update(update_user.into_inner());
    
    // Replace the document
    collection.replace_one(doc! { "_id": object_id }, &user, None).await?;
    Ok(Json(user))
}

#[rocket::delete("/users/<id>")]
pub async fn delete_user(id: String, storage: &State<UserStorage>) -> AppResult<()> {
    let object_id = ObjectId::parse_str(&id)?;
    let collection = get_user_collection(storage);
    
    let result = collection.delete_one(doc! { "_id": object_id }, None).await?;
    if result.deleted_count > 0 {
        Ok(())
    } else {
        Err(AppError::NotFound("User".to_string()))
    }
}

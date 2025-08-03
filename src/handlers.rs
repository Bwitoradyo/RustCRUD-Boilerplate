use rocket::{State, serde::json::Json, response::status, http::Status};
use mongodb::bson::{doc, oid::ObjectId};
use futures::stream::TryStreamExt;
use crate::models::{User, CreateUser, UpdateUser};
use crate::state::{UserStorage, get_user_collection};

#[rocket::get("/users")]
pub async fn get_users(storage: &State<UserStorage>) -> Result<Json<Vec<User>>, Status> {
    let collection = get_user_collection(storage);
    
    match collection.find(doc! {}, None).await {
        Ok(mut cursor) => {
            let mut users = Vec::new();
            while let Some(user) = cursor.try_next().await.map_err(|_| Status::InternalServerError)? {
                users.push(user);
            }
            Ok(Json(users))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[rocket::get("/users/<id>")]
pub async fn get_user(id: String, storage: &State<UserStorage>) -> Result<Json<User>, Status> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let collection = get_user_collection(storage);
    
    match collection.find_one(doc! { "_id": object_id }, None).await {
        Ok(Some(user)) => Ok(Json(user)),
        Ok(None) => Err(Status::NotFound),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[rocket::post("/users", data = "<create_user>")]
pub async fn create_user(create_user: Json<CreateUser>, storage: &State<UserStorage>) -> Result<status::Created<Json<User>>, Status> {
    let mut user = User::new(create_user.name.clone(), create_user.email.clone());
    let collection = get_user_collection(storage);
    
    match collection.insert_one(&user, None).await {
        Ok(result) => {
            user.id = result.inserted_id.as_object_id();
            let location = format!("/users/{}", user.id.unwrap());
            Ok(status::Created::new(location).body(Json(user)))
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

#[rocket::put("/users/<id>", data = "<update_user>")]
pub async fn update_user(id: String, update_user: Json<UpdateUser>, storage: &State<UserStorage>) -> Result<Json<User>, Status> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let collection = get_user_collection(storage);
    
    // Find the user first
    let mut user = match collection.find_one(doc! { "_id": object_id }, None).await {
        Ok(Some(user)) => user,
        Ok(None) => return Err(Status::NotFound),
        Err(_) => return Err(Status::InternalServerError)
    };
    
    // Update the user
    user.update(update_user.into_inner());
    
    // Replace the document
    match collection.replace_one(doc! { "_id": object_id }, &user, None).await {
        Ok(_) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError)
    }
}

#[rocket::delete("/users/<id>")]
pub async fn delete_user(id: String, storage: &State<UserStorage>) -> Result<Status, Status> {
    let object_id = ObjectId::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let collection = get_user_collection(storage);
    
    match collection.delete_one(doc! { "_id": object_id }, None).await {
        Ok(result) => {
            if result.deleted_count > 0 {
                Ok(Status::NoContent)
            } else {
                Ok(Status::NotFound)
            }
        },
        Err(_) => Err(Status::InternalServerError)
    }
}

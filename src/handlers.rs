use rocket::{State, serde::json::Json, response::status, http::Status};
use uuid::Uuid;
use crate::models::{User, CreateUser, UpdateUser};
use crate::state::UserStorage;

#[rocket::get("/users")]
pub fn get_users(storage: &State<UserStorage>) -> Json<Vec<User>> {
    let users = storage.lock().unwrap();
    Json(users.clone())
}

#[rocket::get("/users/<id>")]
pub fn get_user(id: String, storage: &State<UserStorage>) -> Result<Json<User>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let users = storage.lock().unwrap();
    
    users
        .iter()
        .find(|user| user.id == uuid)
        .cloned()
        .map(Json)
        .ok_or(Status::NotFound)
}

#[rocket::post("/users", data = "<create_user>")]
pub fn create_user(create_user: Json<CreateUser>, storage: &State<UserStorage>) -> status::Created<Json<User>> {
    let user = User::new(create_user.name.clone(), create_user.email.clone());
    let mut users = storage.lock().unwrap();
    users.push(user.clone());
    
    let location = format!("/users/{}", user.id);
    status::Created::new(location).body(Json(user))
}

#[rocket::put("/users/<id>", data = "<update_user>")]
pub fn update_user(id: String, update_user: Json<UpdateUser>, storage: &State<UserStorage>) -> Result<Json<User>, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let mut users = storage.lock().unwrap();
    
    users
        .iter_mut()
        .find(|user| user.id == uuid)
        .map(|user| {
            user.update(update_user.into_inner());
            Json(user.clone())
        })
        .ok_or(Status::NotFound)
}

#[rocket::delete("/users/<id>")]
pub fn delete_user(id: String, storage: &State<UserStorage>) -> Result<Status, Status> {
    let uuid = Uuid::parse_str(&id).map_err(|_| Status::BadRequest)?;
    let mut users = storage.lock().unwrap();
    
    if let Some(pos) = users.iter().position(|user| user.id == uuid) {
        users.remove(pos);
        Ok(Status::NoContent)
    } else {
        Ok(Status::NotFound)
    }
}

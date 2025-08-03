use rocket::{Request, Response, response::Responder, http::Status};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: String,
}

#[derive(Debug)]
pub enum AppError {
    DatabaseError(mongodb::error::Error),
    #[allow(dead_code)]
    ValidationError(String),
    NotFound(String),
    InvalidObjectId,
}

pub type AppResult<T> = Result<T, AppError>;

impl From<mongodb::error::Error> for AppError {
    fn from(err: mongodb::error::Error) -> Self {
        AppError::DatabaseError(err)
    }
}

impl From<bson::oid::Error> for AppError {
    fn from(_: bson::oid::Error) -> Self {
        AppError::InvalidObjectId
    }
}

impl<'r> Responder<'r, 'static> for AppError {
    fn respond_to(self, _: &'r Request<'_>) -> rocket::response::Result<'static> {
        let (status, error_type, message) = match self {
            AppError::DatabaseError(err) => (
                Status::InternalServerError,
                "database_error",
                format!("Database operation failed: {}", err)
            ),
            AppError::ValidationError(msg) => (
                Status::BadRequest,
                "validation_error",
                msg
            ),
            AppError::NotFound(resource) => (
                Status::NotFound,
                "not_found",
                format!("{} not found", resource)
            ),
            AppError::InvalidObjectId => (
                Status::BadRequest,
                "invalid_object_id",
                "Invalid ObjectId format".to_string()
            ),
        };

        let error_response = ErrorResponse {
            error: error_type.to_string(),
            message,
        };

        let json_body = rocket::serde::json::to_string(&error_response).unwrap_or_else(|_| "{}".to_string());

        Response::build()
            .status(status)
            .header(rocket::http::ContentType::JSON)
            .sized_body(json_body.len(), std::io::Cursor::new(json_body))
            .ok()
    }
}

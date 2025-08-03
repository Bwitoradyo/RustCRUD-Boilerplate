use std::env;
use crate::error::{AppError, AppResult};

#[derive(Debug, Clone)]
pub struct AppConfig {
    pub mongodb_uri: String,
    pub database_name: String,
    pub max_pool_size: u32,
    pub min_pool_size: u32,
}

impl AppConfig {
    pub fn load() -> AppResult<Self> {
        let mongodb_uri = env::var("MONGODB_URI")
            .unwrap_or_else(|_| "mongodb://localhost:27017".to_string());
        
        let database_name = env::var("DATABASE_NAME")
            .unwrap_or_else(|_| "rocketboiler".to_string());
        
        let max_pool_size = env::var("MAX_POOL_SIZE")
            .unwrap_or_else(|_| "10".to_string())
            .parse()
            .map_err(|_| AppError::ValidationError("Invalid MAX_POOL_SIZE value".to_string()))?;
        
        let min_pool_size = env::var("MIN_POOL_SIZE")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .map_err(|_| AppError::ValidationError("Invalid MIN_POOL_SIZE value".to_string()))?;
        
        if max_pool_size == 0 {
            return Err(AppError::ValidationError("MAX_POOL_SIZE must be greater than 0".to_string()));
        }
        
        if min_pool_size == 0 {
            return Err(AppError::ValidationError("MIN_POOL_SIZE must be greater than 0".to_string()));
        }
        
        if min_pool_size > max_pool_size {
            return Err(AppError::ValidationError("MIN_POOL_SIZE cannot be greater than MAX_POOL_SIZE".to_string()));
        }
        
        Ok(AppConfig {
            mongodb_uri,
            database_name,
            max_pool_size,
            min_pool_size,
        })
    }
}

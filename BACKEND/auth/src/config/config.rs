use crate::constants::env::{
    APP_NAME,
    JWT_SECRET,
    PORT,
};

#[derive(Debug)]
pub struct Config {
    pub app_name: String,
    pub port: String,
    pub jwt_secret: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            app_name: env!(APP_NAME).into(),
            port: env!(PORT).into(),
            jwt_secret: env!(JWT_SECRET).into(),
        }
    }
}

use crate::constants::env::{
    APP_ENVIRONMENT,
    APP_NAME,
    IP_ADDR,
    JWT_SECRET,
    PORT,
};

#[derive(Debug)]
pub struct Config {
    pub app_name: String,
    pub port: String,
    pub jwt_secret: String,
    pub app_environment: String,
    pub ip_addr: String,
}

impl Config {
    pub fn new() -> Self {
        Self {
            app_name: env!(APP_NAME).into(),
            port: env!(PORT).into(),
            jwt_secret: env!(JWT_SECRET).into(),
            app_environment: env!(APP_ENVIRONMENT).into(),
            ip_addr: env!(IP_ADDR).into(),
        }
    }
}

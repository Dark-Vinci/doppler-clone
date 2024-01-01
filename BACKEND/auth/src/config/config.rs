use std::env;
use std::fmt::{self, Display, Formatter};

use crate::constants::env::{APP_ENVIRONMENT, APP_NAME, IP_ADDR, JWT_SECRET, PORT};

#[derive(Debug)]
pub struct Config {
    pub app_name: String,
    pub port: String,
    pub jwt_secret: String,
    pub app_environment: String,
    pub ip_addr: String,
}

impl Display for Config {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let config = format!(
            "[APP_NAME: {0}], [RUNNING_ON_PORT: {1}], [JWT_SECRET: {2}], [IP_ADDR: {3}]",
            self.app_name, self.port, self.jwt_secret, self.ip_addr,
        );

        write!(f, "{}", &config)
    }
}

impl Config {
    pub fn new() -> Self {
        Self {
            app_name: env::var(APP_NAME.into()).into(),
            port: env::var(PORT).into(),
            jwt_secret: env::var(JWT_SECRET).into(),
            app_environment: env::var(APP_ENVIRONMENT).into(),
            ip_addr: env::var(IP_ADDR).into(),
        }
    }
}

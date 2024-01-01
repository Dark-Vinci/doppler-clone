use async_trait::async_trait;

use crate::config::config::Config;

#[async_trait]
pub trait Auth {
    async fn sign_in_with_email(&self) -> String;
    async fn sign_up_with_email(&self) -> String;
    async fn sign_in_with_google(&self) -> String;
    async fn sign_up_with_google(&self) -> String;
}

#[async_trait]
pub trait User {
    async fn get_one(&self) -> String;
    async fn delete(&self) -> String;
    async fn deactivated(&self) -> String;
    async fn get_many(&self) -> Vec<String>;
}

#[async_trait]
pub trait Business {
    async fn get_business(&self) -> String;
    async fn delete_business(&self) -> String;
}

pub trait AppTrait: Auth + User + Business {}

#[derive(Debug)]
pub struct App {
    pub store: String,
    pub downstream: String,
    pub config: Config,
    pub redis: String,
    pub rabbit: String,
}

impl App {
    pub fn new(
        store: String,
        downstream: String,
        config: Config,
        redis: String,
        rabbit: String,
    ) -> Self {
        Self {
            store,
            downstream,
            config,
            redis,
            rabbit,
        }
    }
}

impl AppTrait for App {}

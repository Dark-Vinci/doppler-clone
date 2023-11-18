use crate::config::config::Config;

pub trait Auth {
    async fn sign_in_with_email(&self) -> String;
    async fn sign_up_with_email(&self) -> String;
    async fn sign_in_with_google(&self) -> String;
    async fn sign_up_with_google(&self) -> String;
}

pub trait User {
    async fn get_user(&self) -> String;
    async fn delete_user(&self) -> String;
    async fn deactivated_user(&self) -> String;
    async fn get_users(&self) -> Vec<String>;
}

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

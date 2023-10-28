use async_trait::async_trait;

use crate::app::login::login;
use crate::app::signup::{sign_in, sign_in_with_github, sign_up_with_google};

#[async_trait]
pub trait Application {
    async fn login(&self) -> String;
    async fn sign_in(&self) -> String;
    async fn sign_in_with_google(&self) -> String;
    async fn sign_in_with_github(&self) -> String;
}

#[derive(Debug)]
pub struct App {
    store: String,
    downstream: String,
}

impl App {
    pub fn new(store: String, downstream: String) -> Self {
        Self {
            store,
            downstream,
        }
    }
}

impl Application for App {
    async fn login(&self) -> String {
        return login().await;
    }

    async fn sign_in(&self) -> String {
        return sign_in().await;
    }

    async fn sign_in_with_google(&self) -> String {
        return sign_up_with_google().await;
    }

    async fn sign_in_with_github(&self) -> String {
        return sign_in_with_github().await;
    }
}

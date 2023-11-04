use crate::application::app_data::AppData;
use crate::application::app_trait::Application;
use crate::application::login::login;
use crate::application::signup::{
    sign_in, sign_in_with_github,
    sign_up_with_google,
};

pub struct Applications(AppData);

impl Applications {
    fn new(data: AppData) -> Self {
        Self (data)
    }
}

impl Application for Applications {
    async fn login(&self) -> String {
        return login(&self.0).await;
    }

    async fn sign_in(&self) -> String {
        return sign_in(&self.0).await;
    }

    async fn sign_in_with_google(&self) -> String {
        return sign_up_with_google(&self.0).await;
    }

    async fn sign_in_with_github(&self) -> String {
        return sign_in_with_github(&self.0).await;
    }
}
